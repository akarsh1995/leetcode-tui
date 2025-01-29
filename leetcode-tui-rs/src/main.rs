use color_eyre::Result;
use leetcode_tui_config::CONFIG;
use leetcode_tui_db;
use leetcode_tui_rs::app::App;
use leetcode_tui_rs::utils::update_database_questions;

#[tokio::main]
async fn main() -> Result<()> {
    leetcode_tui_config::init().await?;
    leetcode_tui_db::init(Some(&CONFIG.as_ref().db.path));
    leetcode_core::init(&CONFIG.as_ref().csrftoken, &CONFIG.as_ref().lc_session).await?;
    leetcode_tui_core::init();
    update_database_questions(false).await?;
    App::run().await
}
