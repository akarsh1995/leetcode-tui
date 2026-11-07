use clap::{Parser, Subcommand};
use color_eyre::{
    eyre::{bail, eyre},
    Result,
};
use leetcode_tui_auth::{self as auth, Credentials, LoginOptions};
use leetcode_tui_config::CONFIG;
use leetcode_tui_rs::app::App;
use leetcode_tui_rs::utils::update_database_questions;

#[derive(Parser)]
#[command(
    name = "leetui",
    version,
    author,
    about = "Browse, solve, run and submit LeetCode questions from the terminal"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Sign in to LeetCode and store the session in the config file.
    Login {
        /// Read the session from an already signed in local browser instead of
        /// opening the login window.
        #[arg(long)]
        from_browser: bool,
        /// Browser to read the session from when `--from-browser` is set
        /// (chrome, firefox, safari, ...).
        #[arg(long, value_name = "BROWSER")]
        browser: Option<String>,
        /// Open the login window even when the stored session is still valid.
        #[arg(long)]
        force: bool,
        /// Forget the stored session and the embedded browser profile first, so
        /// you can sign in with a different account.
        #[arg(long)]
        reset: bool,
    },
    /// Show the stored LeetCode session and whether it is still valid.
    Auth,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    leetcode_tui_config::init()?;

    match cli.command {
        Some(Command::Login {
            from_browser,
            browser,
            force,
            reset,
        }) => run_login(from_browser, browser, force, reset),
        Some(Command::Auth) => run_auth_status(),
        None => {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(run_tui())
        }
    }
}

fn run_login(from_browser: bool, browser: Option<String>, force: bool, reset: bool) -> Result<()> {
    // Captured before a reset clears it, so a webview profile that still holds
    // the old session is not captured again straight away.
    let previous = auth::stored();

    if reset {
        auth::persist::clear()?;
        auth::reset_webview_profile(&CONFIG.as_ref().auth)?;
        println!("Cleared the stored session and the embedded browser profile.");
    }

    let runtime = tokio::runtime::Runtime::new()?;

    // Do not open a window just to capture the session we already have: a
    // signed-in webview profile would be picked up before the page is visible.
    if !from_browser && !force && !reset {
        if let Some(existing) = auth::stored() {
            if matches!(runtime.block_on(validate(&existing)), Ok(Some(true))) {
                println!(
                    "Already signed in using the session from {}.",
                    existing.source
                );
                println!("Use `leetui login --reset` to sign in with a different account,");
                println!("or `leetui login --force` to open the login window anyway.");
                return Ok(());
            }
        }
    }

    // A webview profile can hold a session that LeetCode no longer accepts.
    // Rather than failing, throw that profile away and let the user sign in
    // once more.
    let mut retried = false;
    let credentials = loop {
        let credentials = auth::login(
            LoginOptions {
                from_browser,
                browser: browser.clone(),
            },
            previous.as_ref(),
        )?;

        match runtime.block_on(validate(&credentials))? {
            Some(true) => break credentials,
            Some(false)
                if !retried
                    && !from_browser
                    && credentials.source == auth::Source::Webview =>
            {
                retried = true;
                eprintln!("That session is no longer valid, starting a fresh sign-in...");
                auth::reset_webview_profile(&CONFIG.as_ref().auth)?;
            }
            Some(false) => bail!(
                "LeetCode rejected the captured session. Please sign in again, or set a password \
                 via \"Forgot Password?\" on leetcode.com if you normally sign in with Google or GitHub."
            ),
            None => {
                eprintln!(
                    "Could not verify the session (network error or Cloudflare challenge), saving it anyway."
                );
                break credentials;
            }
        }
    };

    auth::persist::save(&credentials)?;
    println!(
        "Signed in to LeetCode using the session from {}.",
        credentials.source
    );

    if !from_browser {
        // The login window is a one-shot process and tearing a webview down
        // after the event loop returns can hang; exit explicitly once the
        // session is safely stored.
        use std::io::Write;
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
        std::process::exit(0);
    }

    Ok(())
}

fn run_auth_status() -> Result<()> {
    let Some(credentials) = auth::stored() else {
        println!("No LeetCode session stored. Run `leetui login` to sign in.");
        return Ok(());
    };

    let runtime = tokio::runtime::Runtime::new()?;
    let status = match runtime.block_on(validate(&credentials)) {
        Ok(Some(true)) => "valid",
        Ok(Some(false)) => "expired or invalid",
        Ok(None) => "unknown (could not reach leetcode.com)",
        Err(error) => {
            println!("Session source: {}", credentials.source);
            println!("Status: unknown ({error})");
            return Ok(());
        }
    };

    println!("Session source: {}", credentials.source);
    println!("Status: {status}");

    if status != "valid" {
        println!("Run `leetui login` to sign in again.");
    }
    Ok(())
}

async fn run_tui() -> Result<()> {
    leetcode_tui_db::init(Some(&CONFIG.as_ref().db.path));

    let credentials = match auth::stored() {
        Some(credentials) => match validate(&credentials).await {
            Ok(Some(true)) => credentials,
            Ok(Some(false)) => {
                eprintln!("Your stored LeetCode session has expired.");
                match sign_in().await {
                    Ok(credentials) => credentials,
                    Err(error) => {
                        eprintln!("Could not sign in: {error}");
                        eprintln!("Continuing with the expired session.");
                        credentials
                    }
                }
            }
            Ok(None) | Err(_) => {
                // Offline, behind a proxy or hit by a Cloudflare challenge:
                // keep going, the TUI works on the cached database.
                log::warn!("could not verify the LeetCode session");
                credentials
            }
        },
        None => match sign_in().await {
            Ok(credentials) => credentials,
            Err(error) => {
                eprintln!("Could not sign in: {error}");
                eprintln!(
                    "Continuing without a LeetCode session; only the cached database is available."
                );
                Credentials::anonymous()
            }
        },
    };

    leetcode_core::init(&credentials.csrf, &credentials.session).await?;
    leetcode_tui_core::init();

    if let Err(error) = update_database_questions(false).await {
        eprintln!("Could not sync the question database: {error}");
        eprintln!("Continuing with the local database.");
    }

    App::run().await
}

/// Opens the login window in a child process and returns the session it stored.
///
/// The webview must own the main thread of its own process: running it inside
/// the TUI process leaves an NSWindow behind that can only be torn down when
/// the TUI exits.
async fn sign_in() -> Result<Credentials> {
    eprintln!("No valid LeetCode session found, opening the login window...");

    let exe = std::env::current_exe()?;
    let status = std::process::Command::new(exe)
        .arg("login")
        .arg("--force")
        .status()?;

    if !status.success() {
        bail!("Sign in failed or the login window was closed.");
    }

    // Read the file the child just wrote rather than `stored()`, which prefers
    // environment variables that may be the expired ones we are replacing.
    auth::from_config_file()
        .or_else(auth::stored)
        .ok_or_else(|| eyre!("The login window finished without storing a session."))
}

async fn validate(credentials: &Credentials) -> Result<Option<bool>> {
    Ok(leetcode_core::validate_credentials(&credentials.csrf, &credentials.session).await?)
}
