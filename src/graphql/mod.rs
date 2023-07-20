use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
pub mod problemset_question_list;
pub mod question_content;
use crate::errors::AppResult;

pub type QuestionContentQuery = question_content::Query;

const LEETCODE_GRAPHQL_ENDPOINT: &str = "https://leetcode.com/graphql";

#[async_trait]
pub trait GQLLeetcodeQuery: Serialize {
    type T: DeserializeOwned;

    fn get_body(&self) -> Value {
        json!(self)
    }

    fn get_endpoint(&self) -> &'static str {
        LEETCODE_GRAPHQL_ENDPOINT
    }

    async fn post(&self, client: &reqwest::Client) -> AppResult<Self::T> {
        Ok(client
            .post(self.get_endpoint())
            .header("Content-Type", "application/json")
            .json(&self.get_body())
            .send()
            .await?
            .json()
            .await?)
    }
}
