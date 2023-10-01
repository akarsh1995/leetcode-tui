use app::app::App;
use app::utils::update_database_questions;
use color_eyre::Result;
use config::error_handling::{initialize_logging, initialize_panic_handler};

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logging()?;
    initialize_panic_handler()?;

    config::init().await?;
    update_database_questions().await?;

    Ok(App::run().await?)
}
