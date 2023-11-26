use super::theme::Theme;
pub use crate::clients::{DB_CLIENT, REQ_CLIENT};
use crate::utils::get_config_dir;
use color_eyre::Result;
pub use log;
use serde::{Deserialize, Serialize};
use shared::RoCell;
use std::{fs::create_dir_all, path::PathBuf};

use std::fs::File;
use std::io::prelude::*;
pub static CONFIG: RoCell<Config> = RoCell::new();

pub fn init() -> Result<()> {
    CONFIG.init({
        let config_dir = get_config_dir();
        let config_file = config_dir.join("config.toml");
        if !config_file.exists() {
            Config::create_default_config(&config_file);
            Config::create_default_solution_dir();
            println!(
                "Please fill in leetcode_sesion and csrftoken in config file @ {}!",
                config_file.display()
            );
            std::process::exit(0);
        }
        let contents = std::fs::read_to_string(&config_file)?;
        toml::from_str(&contents)?
    });
    Ok(())
}

fn get_solutions_dir_path() -> PathBuf {
    get_config_dir().join("solutions")
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub(crate) csrftoken: String,
    pub(crate) lc_session: String,
    #[serde(default)]
    pub(crate) db: Database,
    #[serde(default = "get_solutions_dir_path")]
    pub solutions_dir: PathBuf,
    #[serde(default)]
    pub theme: Theme,
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
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub(crate) conn: String,
    pub(crate) namespace: String,
    pub(crate) database: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            conn: "default".into(),
            namespace: "default".into(),
            database: "ws://localhost:8000".into(),
        }
    }
}
