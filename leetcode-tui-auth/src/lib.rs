//! Credential handling for the LeetCode TUI.
//!
//! LeetCode's login page is protected by reCAPTCHA v3 and Cloudflare Turnstile,
//! so a plain HTTP POST with an email and password cannot work. Instead we run
//! the real login page inside an embedded webview and capture the session
//! cookies it produces. When that is not possible (for example because Google
//! refuses to sign in inside an embedded browser) we fall back to reading the
//! cookies of an already signed in local browser.

use leetcode_tui_config::{AuthConfig, CONFIG};

pub mod persist;

#[cfg(feature = "browser-cookies")]
pub mod browser;

#[cfg(feature = "webview-login")]
pub mod webview;

pub const SESSION_COOKIE: &str = "LEETCODE_SESSION";
pub const CSRF_COOKIE: &str = "csrftoken";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Env,
    Config,
    Browser(String),
    Webview,
    None,
}

impl Source {
    pub fn as_str(&self) -> &str {
        match self {
            Source::Env => "environment variables",
            Source::Config => "config file",
            Source::Browser(browser) => browser,
            Source::Webview => "login window",
            Source::None => "none",
        }
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub session: String,
    pub csrf: String,
    pub source: Source,
}

impl Credentials {
    pub fn new(session: impl Into<String>, csrf: impl Into<String>, source: Source) -> Self {
        Self {
            session: session.into(),
            csrf: csrf.into(),
            source,
        }
    }

    /// Credentials without a session, used to keep the TUI usable with the
    /// cached database when signing in is not possible.
    pub fn anonymous() -> Self {
        Self::new(String::new(), String::new(), Source::None)
    }

    /// Makes sure the values are usable as cookie/header values.
    pub fn is_valid(&self) -> bool {
        !self.session.trim().is_empty() && !self.csrf.trim().is_empty()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("No LeetCode credentials are available. Run `leetui login` to sign in.")]
    NoCredentials,

    #[error("The login window was closed before signing in.")]
    LoginCancelled,

    #[error("Could not open the login window: {0}")]
    Webview(String),

    #[error("Could not read cookies from your browser: {0}")]
    Browser(String),

    #[error("Could not save the session: {0}")]
    Persist(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type AuthResult<T> = Result<T, AuthError>;

/// Reads credentials from `LEETCODE_SESSION` / `LEETCODE_CSRFTOKEN`.
pub fn from_env() -> Option<Credentials> {
    let session = std::env::var(SESSION_COOKIE).ok()?;
    let csrf = std::env::var("LEETCODE_CSRFTOKEN")
        .ok()
        .or_else(|| std::env::var("CSRFTOKEN").ok())?;
    let credentials = Credentials::new(session, csrf, Source::Env);
    credentials.is_valid().then_some(credentials)
}

/// Reads credentials that were persisted in the config file.
pub fn from_config() -> Option<Credentials> {
    let config = CONFIG.as_ref();
    let credentials = Credentials::new(
        config.lc_session.as_ref()?,
        config.csrftoken.as_ref()?,
        Source::Config,
    );
    credentials.is_valid().then_some(credentials)
}

/// Reads credentials straight from the config file on disk.
///
/// Unlike [`from_config`] this picks up a session saved by another process,
/// which is what the TUI needs after `leetui login` ran in a child process.
pub fn from_config_file() -> Option<Credentials> {
    let contents =
        std::fs::read_to_string(leetcode_tui_config::utils::get_config_file_path()).ok()?;
    let document = contents.parse::<toml_edit::DocumentMut>().ok()?;

    let session = document.get("lc_session")?.as_str()?;
    let csrf = document.get("csrftoken")?.as_str()?;

    let credentials = Credentials::new(session, csrf, Source::Config);
    credentials.is_valid().then_some(credentials)
}

/// Credentials available without any user interaction.
pub fn stored() -> Option<Credentials> {
    from_env().or_else(from_config_file).or_else(from_config)
}

/// Tries to read a LeetCode session from the local browsers listed in the
/// config, in order.
pub fn from_browser(browsers: &[String]) -> AuthResult<Option<Credentials>> {
    #[cfg(feature = "browser-cookies")]
    {
        for browser in browsers {
            match browser::read(browser) {
                Ok(Some(credentials)) => return Ok(Some(credentials)),
                Ok(None) => log::debug!("no LeetCode cookies found in {browser}"),
                Err(error) => log::warn!("could not read cookies from {browser}: {error}"),
            }
        }
        Ok(None)
    }

    #[cfg(not(feature = "browser-cookies"))]
    {
        let _ = browsers;
        Ok(None)
    }
}

#[derive(Debug, Default, Clone)]
pub struct LoginOptions {
    /// Skip the login window and read the session from a local browser.
    pub from_browser: bool,
    /// Browser to read the session from when `from_browser` is set.
    pub browser: Option<String>,
}

/// Opens the login window (or reads browser cookies) and returns the captured
/// credentials. The credentials are *not* persisted, see [`persist::save`].
pub fn login(options: LoginOptions) -> AuthResult<Credentials> {
    let auth_config = CONFIG.as_ref().auth.clone();

    if options.from_browser {
        return login_from_browser(&auth_config, options.browser.as_deref());
    }

    #[cfg(feature = "webview-login")]
    {
        if auth_config.webview {
            match webview::run(&auth_config) {
                Ok(credentials) => return Ok(credentials),
                // The user closed the window on purpose, do not silently fall
                // back to something else.
                Err(AuthError::LoginCancelled) => return Err(AuthError::LoginCancelled),
                Err(error) => {
                    log::warn!("login window failed: {error}");
                    eprintln!("The login window could not be used: {error}");
                    eprintln!("Falling back to your local browser session...");
                }
            }
        }
    }

    match login_from_browser(&auth_config, None) {
        Ok(credentials) => Ok(credentials),
        Err(error) => {
            log::warn!("browser fallback failed: {error}");
            eprintln!(
                "Could not sign in automatically.\n\
                 If you sign in with Google or GitHub, set a password via \
                 \"Forgot Password?\" on leetcode.com and try again, or export \
                 your session with `LEETCODE_SESSION`/`LEETCODE_CSRFTOKEN`."
            );
            Err(error)
        }
    }
}

fn login_from_browser(auth_config: &AuthConfig, browser: Option<&str>) -> AuthResult<Credentials> {
    let browsers = match browser {
        Some(browser) => vec![browser.to_string()],
        None => auth_config.browsers.clone(),
    };

    match from_browser(&browsers)? {
        Some(credentials) => Ok(credentials),
        None => Err(AuthError::Browser(format!(
            "no signed in LeetCode session found in: {}",
            browsers.join(", ")
        ))),
    }
}
