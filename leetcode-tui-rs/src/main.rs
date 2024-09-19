use color_eyre::Result;
use leetcode_tui_db;
use leetcode_tui_rs::app::App;
use leetcode_tui_rs::utils::update_database_questions;

#[tokio::main]
async fn main() -> Result<()> {
    leetcode_tui_db::init();

    leetcode_tui_config::init(leetcode_tui_db::DB_BUILDER.as_ref()).await?;

    leetcode_tui_core::init();

    update_database_questions().await?;

    App::run().await
}
