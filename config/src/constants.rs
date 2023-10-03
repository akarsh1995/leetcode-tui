use std::path::PathBuf;
use std::sync::OnceLock;

pub static PROJECT_NAME: OnceLock<String> = OnceLock::new();
pub static DATA_FOLDER: OnceLock<Option<PathBuf>> = OnceLock::new();
pub static CONFIG_FOLDER: OnceLock<Option<PathBuf>> = OnceLock::new();
pub static GIT_COMMIT_HASH: OnceLock<String> = OnceLock::new();
pub static LOG_ENV: OnceLock<String> = OnceLock::new();
pub static LOG_FILE: OnceLock<String> = OnceLock::new();

pub(crate) fn init() {
    let project_name = env!("CARGO_CRATE_NAME").to_uppercase().to_string();

    PROJECT_NAME.get_or_init(|| project_name.clone());

    DATA_FOLDER.get_or_init(|| {
        std::env::var(format!("{}_DATA", project_name.clone()))
            .ok()
            .map(PathBuf::from)
    });
    CONFIG_FOLDER.get_or_init(|| {
        std::env::var(format!("{}_CONFIG", project_name.clone()))
            .ok()
            .map(PathBuf::from)
    });

    GIT_COMMIT_HASH.get_or_init(|| {
        std::env::var(format!("{}_GIT_INFO", project_name.clone()))
            .unwrap_or_else(|_| String::from("UNKNOWN"))
    });

    LOG_ENV.get_or_init(|| format!("{}_LOGLEVEL", project_name.clone()));

    LOG_FILE.get_or_init(|| format!("{}.log", env!("CARGO_PKG_NAME")));
}
