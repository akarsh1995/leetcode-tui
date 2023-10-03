pub mod clients;
pub mod constants;
pub mod error_handling;
pub mod key;
pub mod utils;

pub use clients::{DB_CLIENT, REQ_CLIENT};
use color_eyre::Result;
pub use log;
use serde::Deserialize;
use shared::RoCell;
use utils::get_config_dir;

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

#[derive(Deserialize, Debug)]
pub struct Config {
    csrftoken: String,
    lc_session: String,
    db: Database,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    conn: String,
    namespace: String,
    database: String,
}
