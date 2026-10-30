//! Embedded login window.
//!
//! LeetCode's login page needs JavaScript (reCAPTCHA v3 + Cloudflare Turnstile),
//! so we render the real page in a webview and capture the cookies it sets.
//! This must run on the main thread, which is why the TUI spawns `leetui login`
//! as a separate process instead of calling this directly.

use crate::{AuthError, AuthResult, Credentials, Source, CSRF_COOKIE, SESSION_COOKIE};
use leetcode_tui_config::utils::get_data_dir;
use leetcode_tui_config::AuthConfig;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tao::event::{Event, StartCause, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::platform::run_return::EventLoopExtRunReturn;
use tao::window::WindowBuilder;
use wry::{WebContext, WebViewBuilder};

const LOGIN_URL: &str = "https://leetcode.com/accounts/login/";
const COOKIE_URL: &str = "https://leetcode.com/";
const POLL_INTERVAL: Duration = Duration::from_millis(400);

pub fn run(auth_config: &AuthConfig) -> AuthResult<Credentials> {
    let data_dir = auth_config
        .webview_data_dir
        .clone()
        .unwrap_or_else(|| get_data_dir().join("webview"));

    std::fs::create_dir_all(&data_dir)?;
    restrict_permissions(&data_dir);

    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("LeetCode - Sign in")
        .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 800.0))
        .build(&event_loop)
        .map_err(|error| AuthError::Webview(error.to_string()))?;

    let mut web_context = WebContext::new(Some(data_dir));
    let builder = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_url(LOGIN_URL)
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

    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + POLL_INTERVAL);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) | Event::MainEventsCleared => {
                if let Some(credentials) = read_credentials(&webview) {
                    if let Ok(mut captured) = captured_in_loop.lock() {
                        *captured = Some(credentials);
                    }
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });

    let credentials = captured
        .lock()
        .ok()
        .and_then(|mut captured| captured.take());

    credentials.ok_or(AuthError::LoginCancelled)
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
