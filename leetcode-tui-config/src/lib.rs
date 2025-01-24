pub mod clients;
mod config;
pub mod constants;
pub mod error_handling;
pub mod key;
pub mod theme;
pub mod utils;

pub use crate::config::CONFIG;
use color_eyre::Result;
pub use log;

pub async fn init() -> Result<()> {
    constants::init();
    config::init()?;
    error_handling::initialize_logging()?;
    error_handling::initialize_panic_handler()?;
    Ok(())
}
