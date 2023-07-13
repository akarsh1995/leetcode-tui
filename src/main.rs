use leetcode_tui_rs::models::problem::ProblemSetQuestionListRoot;
use reqwest::{self, cookie::Jar, Url};
use sea_orm::{Database, DatabaseConnection};
use serde_json::{json, Value};
use tracing;
use tracing_subscriber;

use once_cell::sync::Lazy;

const LEETCODE_GRAPHQL_ENDPOINT: &'static str = "https://leetcode.com/graphql/";

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let cookie = "csrftoken=DiVmXR2rlQ0hwWQ5UUsp6v7iROXEYZb4DALhR1b3qwvTWayBZNYNou2oB8YIr2K3; LEETCODE_SESSION=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJfYXV0aF91c2VyX2lkIjoiNTA1MTA2NyIsIl9hdXRoX3VzZXJfYmFja2VuZCI6ImFsbGF1dGguYWNjb3VudC5hdXRoX2JhY2tlbmRzLkF1dGhlbnRpY2F0aW9uQmFja2VuZCIsIl9hdXRoX3VzZXJfaGFzaCI6ImEyMzY4MWU3OWI3MzRhMDY4ZGQxNzFlZjQ4OTAzYjhlZjhkN2ViOGQiLCJpZCI6NTA1MTA2NywiZW1haWwiOiJha2Fyc2guMTk5NS4wMkBnbWFpbC5jb20iLCJ1c2VybmFtZSI6InVzZXI4MTYybCIsInVzZXJfc2x1ZyI6InVzZXI4MTYybCIsImF2YXRhciI6Imh0dHBzOi8vYXNzZXRzLmxlZXRjb2RlLmNvbS91c2Vycy91c2VyODE2MmwvYXZhdGFyXzE2MzM3NzQzODAucG5nIiwicmVmcmVzaGVkX2F0IjoxNjg5MTMyMjc3LCJpcCI6IjE3MS42MC4xNzguMTEzIiwiaWRlbnRpdHkiOiJiOTJiMTVhYzQyZmMzZTc1NzU1ZDY4NWIyOTAwZGExOSIsInNlc3Npb25faWQiOjQyMjc2MzE5fQ.WzJz5Q05NLSsQ9qY6qIbQ5mQYmMloCx6z5g6mJksi2U";
    let url = LEETCODE_GRAPHQL_ENDPOINT.parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(cookie, &url);

    let client = reqwest::ClientBuilder::new();
    client.cookie_store(true).build().unwrap()
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let DATABASE_CLIENT = Database::connect("sqlite://leetcode.sqlite").await.unwrap();

    // Provide variable values here
    let category_slug: String = "".to_string();
    let limit: Option<i32> = Some(1);
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

    let response = CLIENT
        .post(LEETCODE_GRAPHQL_ENDPOINT)
        .json(&graphql_query)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        let response_body: ProblemSetQuestionListRoot = response.json().await?;
        println!("Response: {:?}", response_body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
