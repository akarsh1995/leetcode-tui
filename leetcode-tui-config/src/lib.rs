pub mod clients;
mod config;
pub mod constants;
pub mod error_handling;
pub mod key;
pub mod theme;
pub mod utils;

pub use crate::config::{CONFIG, DB_CLIENT, REQ_CLIENT};
use color_eyre::Result;
pub use log;
use native_db::DatabaseBuilder;

pub async fn init(db_builder: &'static DatabaseBuilder) -> Result<()> {
    constants::init();
    config::init()?;
    clients::init(db_builder).await?;
    error_handling::initialize_logging()?;
    error_handling::initialize_panic_handler()?;
    Ok(())
}
