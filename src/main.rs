use deserializers::question::ProblemSetQuestionList;
use leetcode_tui_rs::config::{self, Config};
use leetcode_tui_rs::db_ops::ModelUtils;
use leetcode_tui_rs::deserializers;
use leetcode_tui_rs::deserializers::question::Question;
use leetcode_tui_rs::graphql::problemset_question_list::Query;
use leetcode_tui_rs::graphql::GQLLeetcodeQuery;
use reqwest::header::{HeaderMap, HeaderValue};
use sea_orm::Database;
use tracing;
use tracing_subscriber;

use once_cell::sync::Lazy;

static CONFIG: Lazy<config::Config> = Lazy::new(|| Config::from_file("./leetcode.config"));

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let csrf = CONFIG.leetcode.csrftoken.as_str();
    let sess = CONFIG.leetcode.leetcode_session.as_str();
    let mut headers = HeaderMap::new();
    headers.append(
        "Cookie",
        HeaderValue::from_str(&format!("LEETCODE_SESSION={sess}; csrftoken={csrf}")).unwrap(),
    );
    reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let DATABASE_CLIENT = Database::connect(CONFIG.db.url.as_str()).await.unwrap();

    let query = Query::default();
    let questions: ProblemSetQuestionList = query.post(&CLIENT).await;
    Question::multi_insert(&DATABASE_CLIENT, questions.questions).await;

    Ok(())
}
