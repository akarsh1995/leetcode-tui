use super::theme::Theme;
use crate::utils::{get_config_file_path, get_default_database_file_path, get_solutions_dir_path};
use color_eyre::Result;
use leetcode_tui_shared::RoCell;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::{fs::create_dir_all, path::PathBuf};
pub static CONFIG: RoCell<Config> = RoCell::new();

pub fn init() -> Result<()> {
    CONFIG.init({
        let config_file = get_config_file_path();
        if !config_file.exists() {
            Config::create_default_config(&config_file);
            Config::create_default_solution_dir();
            println!(
                "Created a default config file @ {}\nYou will be asked to sign in to LeetCode on first run.",
                config_file.display()
            );
        }

        let contents = std::fs::read_to_string(&config_file)?;
        let parsed_config: Config = toml::from_str(&contents)?;

        if parsed_config.db.path.to_str() == Some("") {
            println!(
                "Either fill in field \"db\" in config file or remove the field for default setting\n@ {}",
                config_file.display()
            );
            std::process::exit(0);
        }

        if parsed_config.solutions_dir.to_str() == Some("") {
            println!(
                "Either provide the \"solutions_dir\" path in config file or remove the field for default setting\n@ {}",
                config_file.display()
            );
            std::process::exit(0);
        }

        if !parsed_config.solutions_dir.exists() {
            create_dir_all(parsed_config.solutions_dir.clone())?;
        }

        if !parsed_config.db.path.exists() {
            parsed_config
                .db
                .path
                .parent()
                .map(|parent| create_dir_all(parent));
        }

        parsed_config
    });
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrftoken: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lc_session: Option<String>,
    #[serde(default, skip_serializing)]
    pub db: Database,
    #[serde(default = "get_solutions_dir_path", skip_serializing)]
    pub solutions_dir: PathBuf,
    #[serde(default, skip_serializing)]
    pub theme: Theme,
    #[serde(default)]
    pub auth: AuthConfig,
}

impl Config {
    fn create_default_solution_dir() {
        create_dir_all(get_solutions_dir_path()).unwrap();
    }

    fn create_default_config(config_file: &PathBuf) {
        let config_dir = config_file.as_path().parent().expect(&format!(
            "Cannot get parent of the file path: {}",
            config_file.display()
        ));
        create_dir_all(config_dir)
            .expect(format!("Cannot create config directory @ {}", config_dir.display()).as_str());
        let default_config = Self::default();
        let default_config_str =
            toml::to_string(&default_config).expect("Cannot serialize default config to string.");
        let mut file = File::create(&config_file)
            .expect(format!("Cannot create file @ {}", config_file.display()).as_str());
        file.write_all(default_config_str.as_bytes()).expect(
            format!(
                "Cannot write to the config file @ {}",
                config_file.display()
            )
            .as_str(),
        );
        restrict_permissions(config_file);
    }
}

/// Restricts the config file to the current user, it stores a session token.
fn restrict_permissions(path: &PathBuf) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub path: PathBuf,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: get_default_database_file_path(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_browsers() -> Vec<String> {
    vec![
        "chrome".to_string(),
        "firefox".to_string(),
        "safari".to_string(),
    ]
}

/// Authentication related settings.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthConfig {
    /// Open the embedded login window when no valid session is found.
    #[serde(default = "default_true")]
    pub webview: bool,
    /// Browsers used as a fallback when the embedded login window cannot be
    /// used (for example when Google refuses to sign in inside a webview).
    #[serde(default = "default_browsers")]
    pub browsers: Vec<String>,
    /// Optional directory used to persist the embedded login window profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webview_data_dir: Option<PathBuf>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            webview: default_true(),
            browsers: default_browsers(),
            webview_data_dir: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_legacy_credentials() {
        let config: Config = toml::from_str("lc_session = \"s\"\ncsrftoken = \"c\"").unwrap();
        assert_eq!(config.lc_session.as_deref(), Some("s"));
        assert_eq!(config.csrftoken.as_deref(), Some("c"));
    }

    #[test]
    fn credentials_are_optional() {
        let config: Config = toml::from_str("").unwrap();
        assert!(config.lc_session.is_none());
        assert!(config.csrftoken.is_none());
        assert!(config.auth.webview);
        assert_eq!(config.auth.browsers, ["chrome", "firefox", "safari"]);
    }

    #[test]
    fn default_config_roundtrips() {
        let serialized = toml::to_string(&Config::default()).unwrap();
        let parsed: Config = toml::from_str(&serialized).unwrap();
        assert!(parsed.auth.webview);
        assert!(parsed.lc_session.is_none());
    }
}
