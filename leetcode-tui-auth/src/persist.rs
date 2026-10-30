//! Persists the captured session into the config file.
//!
//! The file is edited with `toml_edit` instead of re-serialising [`Config`],
//! because the `db`, `solutions_dir` and `theme` fields are `skip_serializing`
//! and a full rewrite would silently drop the user's settings.

use crate::{AuthError, AuthResult, Credentials};
use leetcode_tui_config::utils::get_config_file_path;
use std::path::Path;
use toml_edit::DocumentMut;

pub fn save(credentials: &Credentials) -> AuthResult<()> {
    save_to(&get_config_file_path(), credentials)
}

pub fn save_to(path: &Path, credentials: &Credentials) -> AuthResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let existing = std::fs::read_to_string(path).unwrap_or_default();
    let mut document = if existing.trim().is_empty() {
        DocumentMut::new()
    } else {
        existing
            .parse::<DocumentMut>()
            .map_err(|error| AuthError::Persist(error.to_string()))?
    };

    document["lc_session"] = toml_edit::value(credentials.session.clone());
    document["csrftoken"] = toml_edit::value(credentials.csrf.clone());

    std::fs::write(path, document.to_string())?;
    restrict_permissions(path);

    log::info!("saved LeetCode session to {}", path.display());
    Ok(())
}

fn restrict_permissions(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Source;

    fn temp_config(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("leetui-auth-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("config.toml")
    }

    #[test]
    fn keeps_unrelated_settings() {
        let path = temp_config("keeps-settings");
        std::fs::write(
            &path,
            "solutions_dir = \"/tmp/leetui-solutions\"\n\n[theme]\nname = \"default\"\n",
        )
        .unwrap();

        let credentials = Credentials::new("session-value", "csrf-value", Source::Webview);
        save_to(&path, &credentials).unwrap();

        let written = std::fs::read_to_string(&path).unwrap();
        assert!(written.contains("solutions_dir = \"/tmp/leetui-solutions\""));
        assert!(written.contains("name = \"default\""));
        assert!(written.contains("lc_session = \"session-value\""));
        assert!(written.contains("csrftoken = \"csrf-value\""));
    }

    #[test]
    fn creates_the_file_when_missing() {
        let path = temp_config("creates-file");
        let _ = std::fs::remove_file(&path);

        let credentials = Credentials::new("session-value", "csrf-value", Source::Env);
        save_to(&path, &credentials).unwrap();

        let written = std::fs::read_to_string(&path).unwrap();
        assert!(written.contains("lc_session = \"session-value\""));
    }

    #[cfg(unix)]
    #[test]
    fn restricts_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let path = temp_config("permissions");
        let credentials = Credentials::new("session-value", "csrf-value", Source::Env);
        save_to(&path, &credentials).unwrap();

        let mode = std::fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }
}
