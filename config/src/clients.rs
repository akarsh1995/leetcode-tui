use color_eyre::Result;
use shared::RoCell;

use reqwest::Client;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;

use reqwest::header::{HeaderMap, HeaderValue};
pub use surrealdb::engine::any::connect;

use crate::CONFIG;
pub type Db = Surreal<Any>;
pub static DB_CLIENT: RoCell<Db> = RoCell::new();
pub static REQ_CLIENT: RoCell<Client> = RoCell::new();

pub(crate) async fn init() -> Result<()> {
    DB_CLIENT.init({
        let db = connect(&CONFIG.as_ref().db.conn).await?;
        db.use_ns(&CONFIG.as_ref().db.namespace)
            .use_db(&CONFIG.as_ref().db.database)
            .await?;
        db
    });
    REQ_CLIENT
        .init(build_reqwest_client(&CONFIG.as_ref().csrftoken, &CONFIG.as_ref().lc_session).await?);
    Ok(())
}

pub async fn build_reqwest_client(csrf: &str, sess: &str) -> Result<Client> {
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
        .build()?;
    Ok(client)
}
