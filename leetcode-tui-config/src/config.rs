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
                "Please fill in leetcode_sesion and csrftoken in config file \n @ {}",
                config_file.display()
            );
            std::process::exit(0);
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
                "Either provide the \"solutions_dir\" path in config file or remove the field for default\n@ {}",
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
    pub csrftoken: String,
    pub lc_session: String,
    #[serde(default, skip_serializing)]
    pub db: Database,
    #[serde(default = "get_solutions_dir_path", skip_serializing)]
    pub solutions_dir: PathBuf,
    #[serde(default, skip_serializing)]
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
    pub path: PathBuf,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: get_default_database_file_path(),
        }
    }
}
