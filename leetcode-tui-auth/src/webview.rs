//! Embedded login window.
//!
//! LeetCode's login page needs JavaScript (reCAPTCHA v3 + Cloudflare Turnstile),
//! so we render the real page in a webview and capture the cookies it sets.
//! This must run on the main thread, which is why the TUI spawns `leetui login`
//! as a separate process instead of calling this directly.

use crate::{AuthError, AuthResult, Credentials, Source, CSRF_COOKIE, SESSION_COOKIE};
use leetcode_tui_config::utils::get_data_dir;
use leetcode_tui_config::AuthConfig;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tao::event::{Event, StartCause, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::platform::run_return::EventLoopExtRunReturn;
use tao::window::WindowBuilder;
use wry::{WebContext, WebViewBuilder};

const LOGIN_URL: &str = "https://leetcode.com/accounts/login/";
const COOKIE_URL: &str = "https://leetcode.com/";
const POLL_INTERVAL: Duration = Duration::from_millis(250);
/// Give the page a moment to settle after the session cookie changes, so the
/// webview is not torn down in the middle of a navigation.
const SETTLE_DELAY: Duration = Duration::from_millis(600);

/// Shown on top of LeetCode's page so the window does not look stuck: the
/// session is captured automatically once it changes, but closing the window
/// also finishes the flow.
const HINT_SCRIPT: &str = r#"
(function () {
  function mount() {
    if (document.getElementById('leetui-hint')) return;
    var hint = document.createElement('div');
    hint.id = 'leetui-hint';
    hint.textContent = 'Signed in? Close this window to finish.';
    hint.style.cssText = [
      'position:fixed', 'left:50%', 'transform:translateX(-50%)', 'bottom:16px',
      'z-index:2147483647', 'background:#111', 'color:#fff',
      'font:13px -apple-system,system-ui,sans-serif', 'padding:8px 14px',
      'border-radius:999px', 'box-shadow:0 4px 16px rgba(0,0,0,.35)',
      'pointer-events:none', 'opacity:.92'
    ].join(';');
    document.documentElement.appendChild(hint);
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', mount);
  } else {
    mount();
  }
})();
"#;

/// Opens the login window and returns the session it produces.
///
/// `previous` is the session we already had. Cookies identical to it are
/// ignored, otherwise a still-signed-in webview profile would be captured
/// instantly and the window would close before the user could do anything.
pub fn run(auth_config: &AuthConfig, previous: Option<&Credentials>) -> AuthResult<Credentials> {
    let data_dir = auth_config
        .webview_data_dir
        .clone()
        .unwrap_or_else(|| get_data_dir().join("webview"));

    std::fs::create_dir_all(&data_dir)?;
    restrict_permissions(&data_dir);

    let previous_session = previous.map(|credentials| credentials.session.clone());

    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("LeetCode - Sign in")
        .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 800.0))
        .build(&event_loop)
        .map_err(|error| AuthError::Webview(error.to_string()))?;

    let mut web_context = WebContext::new(Some(data_dir));
    let builder = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_url(LOGIN_URL)
        .with_initialization_script(HINT_SCRIPT)
        .with_devtools(cfg!(debug_assertions));

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let webview = builder
        .build(&window)
        .map_err(|error| AuthError::Webview(error.to_string()))?;

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window
            .default_vbox()
            .ok_or_else(|| AuthError::Webview("could not create the GTK container".to_string()))?;
        builder
            .build_gtk(vbox)
            .map_err(|error| AuthError::Webview(error.to_string()))?
    };

    window.set_focus();

    let captured: Arc<Mutex<Option<Credentials>>> = Arc::new(Mutex::new(None));
    let captured_in_loop = Arc::clone(&captured);
    let mut settle_at: Option<Instant> = None;

    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + POLL_INTERVAL);

        match event {
            // Closing the window is an explicit "I'm done": capture whatever
            // the page produced, but only if it is a new session.
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                if let Some(credentials) = read_credentials(&webview) {
                    if is_new_session(&credentials, previous_session.as_deref()) {
                        if let Ok(mut captured) = captured_in_loop.lock() {
                            *captured = Some(credentials);
                        }
                    }
                }
                *control_flow = ControlFlow::Exit;
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) | Event::MainEventsCleared => {
                if settle_at.is_some_and(|deadline| Instant::now() >= deadline) {
                    window.set_visible(false);
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if settle_at.is_none() {
                    if let Some(credentials) = read_credentials(&webview) {
                        if is_new_session(&credentials, previous_session.as_deref()) {
                            if let Ok(mut captured) = captured_in_loop.lock() {
                                *captured = Some(credentials);
                            }
                            let deadline = Instant::now() + SETTLE_DELAY;
                            settle_at = Some(deadline);
                            *control_flow = ControlFlow::WaitUntil(deadline);
                        }
                    }
                }
            }
            _ => {}
        }
    });

    captured
        .lock()
        .ok()
        .and_then(|mut captured| captured.take())
        .ok_or(AuthError::LoginCancelled)
}

/// Removes the embedded browser profile so the next login starts signed out.
pub fn reset_profile(auth_config: &AuthConfig) -> AuthResult<()> {
    for path in profile_paths(auth_config) {
        let result = if path.is_dir() {
            std::fs::remove_dir_all(&path)
        } else if path.exists() {
            std::fs::remove_file(&path)
        } else {
            continue;
        };

        match result {
            Ok(()) => log::info!("removed webview profile {}", path.display()),
            Err(error) => {
                // Not fatal: the session diffing still protects us from
                // capturing a stale session straight away.
                log::warn!("could not remove {}: {error}", path.display());
                eprintln!("Could not remove {}: {error}", path.display());
            }
        }
    }
    Ok(())
}

fn profile_paths(auth_config: &AuthConfig) -> Vec<PathBuf> {
    let mut paths = vec![auth_config
        .webview_data_dir
        .clone()
        .unwrap_or_else(|| get_data_dir().join("webview"))];

    // On macOS `WebContext`'s data directory is ignored: WKWebView keeps its
    // persistent store under ~/Library, keyed by the process name.
    #[cfg(target_os = "macos")]
    {
        if let Some(name) = process_name() {
            if let Some(home) = std::env::var_os("HOME") {
                let home = PathBuf::from(home);
                paths.push(home.join("Library").join("WebKit").join(&name));
                paths.push(
                    home.join("Library")
                        .join("HTTPStorages")
                        .join(format!("{name}.binarycookies")),
                );
            }
        }
    }

    paths
}

fn process_name() -> Option<String> {
    let name = std::env::current_exe()
        .ok()?
        .file_stem()?
        .to_string_lossy()
        .into_owned();
    (!name.is_empty() && !name.contains('/') && !name.contains('\\')).then_some(name)
}

fn is_new_session(credentials: &Credentials, previous_session: Option<&str>) -> bool {
    match previous_session {
        Some(previous) => credentials.session != previous,
        None => true,
    }
}

fn read_credentials(webview: &wry::WebView) -> Option<Credentials> {
    let cookies = match webview.cookies_for_url(COOKIE_URL) {
        Ok(cookies) => cookies,
        Err(error) => {
            log::debug!("could not read webview cookies: {error}");
            return None;
        }
    };

    let mut session = None;
    let mut csrf = None;

    for cookie in cookies {
        match cookie.name() {
            SESSION_COOKIE => session = Some(cookie.value().to_string()),
            CSRF_COOKIE => csrf = Some(cookie.value().to_string()),
            _ => {}
        }
    }

    session
        .zip(csrf)
        .map(|(session, csrf)| Credentials::new(session, csrf, Source::Webview))
}

fn restrict_permissions(path: &std::path::Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700));
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
}
