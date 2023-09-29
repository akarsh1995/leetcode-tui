use app::app;
use color_eyre::Result;
use leetcode_db::connect;
use shared::utils::{initialize_logging, initialize_panic_handler};

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logging()?;
    initialize_panic_handler()?;
    // let db = connect("mem://").await.unwrap();
    let db = connect("ws://localhost:8000").await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    Ok(app::App::run(&db).await?)
}
