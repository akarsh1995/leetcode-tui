//! Fallback: read the LeetCode session from an already signed in browser.

use crate::{AuthError, AuthResult, Credentials, Source, CSRF_COOKIE, SESSION_COOKIE};

const LEETCODE_DOMAIN: &str = "leetcode.com";

/// Reads the LeetCode cookies of a single browser.
///
/// Returns `Ok(None)` when the browser could be read but has no LeetCode
/// session, and `Err` when the browser store could not be read at all.
pub fn read(browser: &str) -> AuthResult<Option<Credentials>> {
    let request = rookie_cookies::ExtractRequest::browser(browser)
        .domains(Some(vec![LEETCODE_DOMAIN.to_string()]))
        .include_session()
        // Do not inject into the browser process on Windows; endpoint security
        // products flag that behaviour and it is not worth it for us.
        .app_bound(rookie_cookies::AppBoundPolicy::Disabled);

    let cookies =
        rookie_cookies::extract(request).map_err(|error| AuthError::Browser(error.to_string()))?;

    let mut session = None;
    let mut csrf = None;

    for cookie in cookies {
        match cookie.name.as_str() {
            SESSION_COOKIE => session = Some(cookie.value),
            CSRF_COOKIE => csrf = Some(cookie.value),
            _ => {}
        }
    }

    Ok(session.zip(csrf).map(|(session, csrf)| {
        Credentials::new(session, csrf, Source::Browser(browser.to_string()))
    }))
}
