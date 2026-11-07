pub mod errors;
pub mod graphql;
pub mod types;
use errors::AppResult;
pub use graphql::client::GQLLeetcodeRequest;
pub use graphql::query::problemset_question_list::Query as QuestionRequest;
pub use graphql::query::question_content::Query as QuestionContentRequest;
pub use graphql::query::run_code::RunCodeRequest;
pub use graphql::query::submit_code::SubmitCodeRequest;
pub use graphql::query::EditorDataRequest;
pub use reqwest::header::{HeaderMap, HeaderValue};
pub use reqwest::Client;
use std::sync::{OnceLock, RwLock};
use std::time::Duration;
pub use types::editor_data::QuestionData as EditorDataResponse;
pub use types::problemset_question_list::Root as QuestionResponse;
pub use types::rest_problems_all::ProblemsAllResponse;

pub static REQ_CLIENT: OnceLock<RwLock<reqwest::Client>> = OnceLock::new();

/// GraphQL query used to check whether the given credentials are still valid.
const USER_STATUS_QUERY: &str =
    r#"{"query":"query userStatus { userStatus { isSignedIn username } }"}"#;

pub async fn init(csrf: &str, sess: &str) -> AppResult<()> {
    reinit(csrf, sess)
}

/// (Re)builds the shared HTTP client with the given credentials.
///
/// Safe to call multiple times: the client is swapped in place, which allows
/// the TUI to recover from an expired session without restarting.
pub fn reinit(csrf: &str, sess: &str) -> AppResult<()> {
    let client = build_reqwest_client(csrf, sess)?;
    match REQ_CLIENT.get() {
        Some(client_lock) => {
            *client_lock.write().expect("REQ_CLIENT lock is poisoned") = client;
        }
        None => {
            let _ = REQ_CLIENT.set(RwLock::new(client));
        }
    }
    Ok(())
}

pub(crate) fn get_client() -> Client {
    REQ_CLIENT
        .get()
        .expect("Client not initialized")
        .read()
        .expect("REQ_CLIENT lock is poisoned")
        .clone()
}

/// Checks the given credentials against LeetCode without touching the shared
/// client.
///
/// Returns:
/// - `Ok(Some(true))` when the session is signed in,
/// - `Ok(Some(false))` when LeetCode rejected the session,
/// - `Ok(None)` when the answer is inconclusive (for example a Cloudflare
///   challenge or a proxy blocking the request).
pub async fn validate_credentials(csrf: &str, sess: &str) -> AppResult<Option<bool>> {
    let client = build_reqwest_client(csrf, sess)?;
    let response = client
        .post("https://leetcode.com/graphql")
        .header("Content-Type", "application/json")
        .body(USER_STATUS_QUERY)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if status.as_u16() == 403 {
        // LeetCode answers an expired session with a JSON body, while a
        // Cloudflare challenge is HTML. Only the former means "signed out".
        if body.trim_start().starts_with('<') {
            return Ok(None);
        }
        return Ok(Some(false));
    }

    if !status.is_success() {
        return Ok(None);
    }

    let parsed: serde_json::Value = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(_) => return Ok(None),
    };

    Ok(parsed
        .pointer("/data/userStatus/isSignedIn")
        .and_then(serde_json::Value::as_bool))
}

pub fn build_reqwest_client(csrf: &str, sess: &str) -> AppResult<Client> {
    let mut headers = HeaderMap::new();
    let header_k_v = [
        (
            "Cookie",
            format!("LEETCODE_SESSION={sess}; csrftoken={csrf}"),
        ),
        ("Content-Type", "application/json".to_string()),
        ("x-csrftoken", csrf.to_string()),
        ("Origin", "https://leetcode.com".to_string()),
        ("Referer", "https://leetcode.com".to_string()),
        ("Connection", "keep-alive".to_string()),
    ];

    for (key, value) in header_k_v {
        headers.append(key, HeaderValue::from_str(value.as_str())?);
    }

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()?;
    Ok(client)
}

pub async fn fetch_all_problems_rest() -> AppResult<ProblemsAllResponse> {
    let response = get_client()
        .get("https://leetcode.com/api/problems/all")
        .send()
        .await?;

    if response.status().as_u16() == 403 {
        return Err(errors::LcAppError::CookiesExpiredError);
    } else if response.status().as_u16() != 200 {
        return Err(errors::LcAppError::StatusCodeError {
            code: response.status().to_string(),
            contents: response.text().await?,
        });
    }

    let result = response.text().await?;
    let parsed: ProblemsAllResponse = serde_json::from_str(&result)?;
    Ok(parsed)
}
