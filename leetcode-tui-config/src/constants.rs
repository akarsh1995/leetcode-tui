use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

pub static PROJECT_NAME: OnceLock<String> = OnceLock::new();
pub static DATA_FOLDER: OnceLock<Option<PathBuf>> = OnceLock::new();
pub static CONFIG_FOLDER: OnceLock<Option<PathBuf>> = OnceLock::new();
pub static GIT_COMMIT_HASH: OnceLock<String> = OnceLock::new();
pub static LOG_ENV: OnceLock<String> = OnceLock::new();
pub static LOG_FILE: OnceLock<String> = OnceLock::new();
pub static EDITOR: OnceLock<String> = OnceLock::new();

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

    EDITOR.get_or_init(|| {
        if let Ok(env_editor) = std::env::var("EDITOR") {
            env_editor
        } else if is_executable_in_path("nvim") {
            "nvim".into()
        } else if is_executable_in_path("vim") {
            "vim".into()
        } else if is_executable_in_path("nano") {
            "nano".into()
        } else {
            "code".into()
        }
    });
}

fn is_executable_in_path(executable_name: &str) -> bool {
    if let Some(path_var) = env::var_os("PATH") {
        if let Some(paths) = env::split_paths(&path_var).collect::<Vec<_>>().first() {
            for path_dir in paths.iter() {
                let path_dir: &Path = path_dir.as_ref();
                let executable_path = path_dir.join(executable_name);
                if executable_path.exists() && executable_path.is_file() {
                    return true;
                }
            }
        }
    }
    false
}
