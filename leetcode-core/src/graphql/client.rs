use crate::errors::{AppResult, LcAppError};
use crate::get_client;
use async_trait::async_trait;
use lru::LruCache;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::RwLock;
use std::{num::NonZeroUsize, sync::OnceLock};

pub static CACHE: OnceLock<RwLock<LruCache<u64, String>>> = OnceLock::new();

fn get_cache<'a>() -> &'a RwLock<LruCache<u64, String>> {
    CACHE.get_or_init(|| RwLock::new(LruCache::new(NonZeroUsize::new(20).unwrap())))
}

fn hash_string(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

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

    fn use_cache(&self) -> bool {
        false
    }

    fn get_query_hash(&self) -> u64 {
        hash_string(format!("{}{}", self.get_endpoint(), self.get_body()).as_str())
    }

    async fn send(&self) -> AppResult<Self::T> {
        if self.use_cache() {
            let mut c = get_cache().write().unwrap();
            if let Some(value) = c.get(&self.get_query_hash()) {
                return Ok(serde_json::from_str(value.as_str())?);
            };
        }

        let request = if self.is_post() {
            get_client()
                .post(self.get_endpoint())
                .json(&self.get_body())
        } else {
            get_client().get(self.get_endpoint())
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
        let result = response.text().await?;

        if self.use_cache() {
            let mut c = get_cache().write().unwrap();
            c.put(self.get_query_hash(), result.clone());
        }
        match serde_json::from_str(result.as_str()) {
            Ok(parsed_message) => Ok(parsed_message),
            Err(e) => {
                log::debug!("{}\n{}", &e, result.as_str());
                Err(LcAppError::DeserializeError(e))
            }
        }
    }
}
