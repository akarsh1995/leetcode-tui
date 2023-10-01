pub mod key;

pub use clients::{DB_CLIENT, REQ_CLIENT};
use color_eyre::Result;
use serde::Deserialize;
use shared::RoCell;
use toml;

pub static CONFIG: RoCell<Config> = RoCell::new();

pub async fn init() -> Result<()> {
    CONFIG.init({
        let contents = std::fs::read_to_string("config.toml")?;
        toml::from_str(&contents)?
    });
    clients::init().await
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

pub mod clients;
