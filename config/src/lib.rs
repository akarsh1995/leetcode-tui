pub mod clients;
pub mod constants;
pub mod error_handling;
pub mod key;
pub mod utils;

use std::path::PathBuf;

pub use clients::{DB_CLIENT, REQ_CLIENT};
use color_eyre::Result;
pub use log;
use serde::Deserialize;
use shared::RoCell;
use utils::get_config_dir;

// pub mod fileops {
//     use std::{
//         fs::read_dir,
//         path::{Path, PathBuf},
//     };

//     impl From<PathBuf> for SolutionDir {
//         fn from(value: PathBuf) -> Self {
//             Self {
//                 path: value,
//                 files: vec![],
//             }
//         }
//     }

//     struct SolutionDir {
//         path: PathBuf,
//         files: Vec<PathBuf>,
//     }

//     impl SolutionDir {
//         fn populate_files(&mut self, filter: &dyn Fn(&Path) -> bool) {
//             for p in read_dir(self.path.as_path()).unwrap() {
//                 if let Ok(_p) = p {
//                     if _p.path().is_file() {}
//                 }
//             }
//         }
//     }
// }

pub static CONFIG: RoCell<Config> = RoCell::new();

pub async fn init() -> Result<()> {
    constants::init();
    CONFIG.init({
        let config_dir = get_config_dir();
        let config_file = config_dir.join("config.toml");
        let contents = std::fs::read_to_string(config_file)?;
        toml::from_str(&contents)?
    });

    clients::init().await?;
    error_handling::initialize_logging()?;
    error_handling::initialize_panic_handler()
}

fn get_solutions_dir_path() -> PathBuf {
    get_config_dir().join("solutions")
}

#[derive(Deserialize, Debug)]
pub struct Config {
    csrftoken: String,
    lc_session: String,
    db: Database,
    #[serde(default = "get_solutions_dir_path")]
    pub solutions_dir: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    conn: String,
    namespace: String,
    database: String,
}
