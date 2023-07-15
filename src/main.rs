use leetcode_tui_rs::config::{self, Config};
use leetcode_tui_rs::deserializers::question::{ProblemSetQuestionListQuery, Question};
use leetcode_tui_rs::queries::question::ModelUtils;
use reqwest::header::HeaderMap;
use reqwest::{self, cookie::Jar, Url};
use sea_orm::Database;
use serde_json::{json, Value};
use tracing;
use tracing_subscriber;

use once_cell::sync::Lazy;

const LEETCODE_GRAPHQL_ENDPOINT: &'static str = "https://leetcode.com/graphql/";

static CONFIG: Lazy<config::Config> = Lazy::new(|| Config::from_file("./leetcode.config"));

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let DATABASE_CLIENT = Database::connect(CONFIG.db.url.as_str()).await.unwrap();

    // Provide variable values here
    let category_slug: String = "".to_string();
    let limit: Option<i32> = Some(20);
    let skip: Option<i32> = Some(0);
    let filters: Value = json!({});

    let graphql_query = json!({
        "query": r#"query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
            problemsetQuestionList: questionList(
                categorySlug: $categorySlug
                limit: $limit
                skip: $skip
                filters: $filters
            ) {
                total: totalNum
                questions: data {
                    acRate
                    difficulty
                    freqBar
                    frontendQuestionId: questionFrontendId
                    isFavor
                    paidOnly: isPaidOnly
                    status
                    title
                    titleSlug
                    topicTags {
                        name
                        id
                        slug
                    }
                    hasSolution
                    hasVideoSolution
                }
            }
        }"#,
        "variables": {
            "categorySlug": category_slug,
            "limit": limit,
            "skip": skip,
            "filters": filters,
        }
    });

    let csrf = CONFIG.leetcode.csrftoken.as_str();
    let sess = CONFIG.leetcode.leetcode_session.as_str();

    let response = CLIENT
        .post(LEETCODE_GRAPHQL_ENDPOINT)
        .header(
            "Cookie",
            format!("LEETCODE_SESSION={sess}; csrftoken={csrf}"),
        )
        .header("Content-Type", "application/json")
        .json(&graphql_query)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        let response_body: ProblemSetQuestionListQuery = response.json().await?;
        let questions = response_body.get_questions();
        dbg!(&questions);
        Question::multi_insert(&DATABASE_CLIENT, questions).await;
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
