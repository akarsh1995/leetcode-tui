use app::app::App;
use app::utils::update_database_questions;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    config::init().await?;
    app_core::init();

    update_database_questions().await?;

    App::run().await
}
