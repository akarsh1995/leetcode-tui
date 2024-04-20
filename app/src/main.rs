use app::app::App;
use app::utils::update_database_questions;
use color_eyre::Result;
use leetcode_db;

#[tokio::main]
async fn main() -> Result<()> {
    leetcode_db::init();

    config::init(leetcode_db::DB_BUILDER.as_ref()).await?;

    app_core::init();

    update_database_questions().await?;

    App::run().await
}
