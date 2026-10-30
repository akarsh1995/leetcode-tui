use clap::{Parser, Subcommand};
use color_eyre::{eyre::bail, Result};
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
        /// Browser to read the session from (chrome, firefox, safari, ...).
        #[arg(long, value_name = "BROWSER")]
        browser: Option<String>,
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
        }) => run_login(from_browser, browser),
        Some(Command::Auth) => run_auth_status(),
        None => {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(run_tui())
        }
    }
}

fn run_login(from_browser: bool, browser: Option<String>) -> Result<()> {
    let credentials = auth::login(LoginOptions {
        from_browser,
        browser,
    })?;

    let runtime = tokio::runtime::Runtime::new()?;
    match runtime.block_on(validate(&credentials))? {
        Some(true) => {}
        Some(false) => bail!(
            "LeetCode rejected the captured session. Please sign in again, or set a password \
             via \"Forgot Password?\" on leetcode.com if you normally sign in with Google or GitHub."
        ),
        None => eprintln!(
            "Could not verify the session (network error or Cloudflare challenge), saving it anyway."
        ),
    }

    auth::persist::save(&credentials)?;
    println!(
        "Signed in to LeetCode using the session from {}.",
        credentials.source
    );
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

/// Opens the login window (must be called from the main thread) and stores the
/// captured session.
async fn sign_in() -> Result<Credentials> {
    eprintln!("No valid LeetCode session found, opening the login window...");
    let credentials = auth::login(LoginOptions::default())?;

    match validate(&credentials).await {
        Ok(Some(true)) => {}
        Ok(Some(false)) => bail!("LeetCode did not accept the captured session."),
        Ok(None) | Err(_) => eprintln!(
            "Could not verify the session (network error or Cloudflare challenge), saving it anyway."
        ),
    }

    auth::persist::save(&credentials)?;
    eprintln!(
        "Signed in to LeetCode using the session from {}.",
        credentials.source
    );
    Ok(credentials)
}

async fn validate(credentials: &Credentials) -> Result<Option<bool>> {
    Ok(leetcode_core::validate_credentials(&credentials.csrf, &credentials.session).await?)
}
