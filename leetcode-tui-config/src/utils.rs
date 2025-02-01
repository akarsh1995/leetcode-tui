use crate::constants::*;
use std::path::PathBuf;

use directories::ProjectDirs;

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "akarsh", "leetui")
}

/// Returns the data directory path
/// Examples:
/// - Windows: C:\Users\<username>\AppData\Local\leetcode-tui
/// - macOS: /Users/<username>/Library/Application Support/leetcode-tui
/// - Linux: /home/<username>/.local/share/leetcode-tui
pub fn get_data_dir() -> PathBuf {
    project_directory()
        .map(|proj| proj.data_local_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".leetcode-tui"))
}

/// Returns the config directory path
/// Examples:
/// - Windows: C:\Users\<username>\AppData\Roaming\leetcode-tui
/// - macOS: /Users/<username>/Library/Application Support/leetcode-tui
/// - Linux: /home/<username>/.config/leetcode-tui
pub fn get_config_dir() -> PathBuf {
    project_directory()
        .map(|proj| proj.config_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".leetcode-tui"))
}

/// Returns the config file path
/// Examples:
/// - Windows: C:\Users\<username>\AppData\Roaming\leetcode-tui\config.toml
/// - macOS: /Users/<username>/Library/Application Support/leetcode-tui/config.toml
/// - Linux: /home/<username>/.config/leetcode-tui/config.toml
pub fn get_config_file_path() -> PathBuf {
    get_config_dir().join("config.toml")
}

/// Returns the solutions directory path
/// Examples:
/// - Windows: C:\Users\<username>\AppData\Local\leetcode-tui\solutions
/// - macOS: /Users/<username>/Library/Application Support/leetcode-tui/solutions
/// - Linux: /home/<username>/.local/share/leetcode-tui/solutions
pub(crate) fn get_solutions_dir_path() -> PathBuf {
    get_data_dir().join("solutions")
}

/// Returns the default database file path
/// Examples:
/// - Windows: C:\Users\<username>\AppData\Local\leetcode-tui\questions.db
/// - macOS: /Users/<username>/Library/Application Support/leetcode-tui/questions.db
/// - Linux: /home/<username>/.local/share/leetcode-tui/questions.db
pub(crate) fn get_default_database_file_path() -> PathBuf {
    get_data_dir().join("questions.db")
}

pub fn version() -> String {
    let author = clap::crate_authors!();
    let commit_hash = GIT_COMMIT_HASH.get().unwrap().clone();
    let config_dir_path = get_config_dir().display().to_string();
    let data_dir_path = get_data_dir().display().to_string();

    format!(
        "\
{commit_hash}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
    )
}
