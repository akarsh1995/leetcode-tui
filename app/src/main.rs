use app::app;
use color_eyre::Result;
use leetcode_db::connect;

#[tokio::main]
async fn main() -> Result<()> {
    // let db = connect("mem://").await.unwrap();
    let db = connect("ws://localhost:8000").await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    Ok(app::App::run(&db).await?)
}
