use crate::errors::{AppResult, LcAppError};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

#[async_trait]
pub trait GQLLeetcodeRequest: Serialize + Sync {
    type T: DeserializeOwned;

    fn get_body(&self) -> Value {
        json!(self)
    }

    fn is_post(&self) -> bool {
        true
    }

    /// Default graphql endpoint
    fn get_endpoint(&self) -> String {
        "https://leetcode.com/graphql".to_string()
    }

    async fn send(&self, client: &reqwest::Client) -> AppResult<Self::T> {
        let request = if self.is_post() {
            client.post(self.get_endpoint()).json(&self.get_body())
        } else {
            client.get(self.get_endpoint())
        };
        let response = request
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().as_u16() == 403 {
            return Err(LcAppError::CookiesExpiredError);
        } else if response.status().as_u16() != 200 {
            return Err(LcAppError::StatusCodeError {
                code: response.status().to_string(),
                contents: response.text().await?,
            });
        }

        Ok(response.json().await?)
    }
}
