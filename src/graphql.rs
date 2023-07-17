use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
pub mod problemset_question_list;
pub mod question_content;

pub type QuestionContentQuery = question_content::Query;

const LEETCODE_GRAPHQL_ENDPOINT: &'static str = "https://leetcode.com/graphql/";

#[async_trait]
pub trait GQLLeetcodeQuery: Serialize {
    fn get_body(&self) -> Value {
        json!(self)
    }

    fn get_endpoint(&self) -> &'static str {
        LEETCODE_GRAPHQL_ENDPOINT
    }

    async fn post<T: DeserializeOwned>(&self, client: &reqwest::Client) -> T {
        client
            .post(self.get_endpoint())
            .header("Content-Type", "application/json")
            .json(&self.get_body())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}
