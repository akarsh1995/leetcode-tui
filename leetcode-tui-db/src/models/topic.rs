use crate::{
    errors::{DBResult, DbErr},
    get_conn, run_sync, DbQuestion,
};
use libsql::params;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct DbTopic {
    pub slug: String,
}

impl Hash for DbTopic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.slug.hash(state);
    }
}

impl DbTopic {
    pub fn new(slug: &str) -> Self {
        Self { slug: slug.into() }
    }

    pub fn fetch_all() -> DBResult<Vec<DbTopic>> {
        run_sync(async {
            let conn = get_conn();
            let mut rows = conn
                .query("SELECT slug FROM topics ORDER BY slug ASC;", ())
                .await?;
            let mut topics = Vec::new();
            while let Some(row) = rows.next().await? {
                let slug: String = row.get(0)?;
                topics.push(DbTopic::new(&slug));
            }
            Ok(topics)
        })
    }

    pub fn fetch_questions(&self) -> DBResult<Vec<DbQuestion>> {
        DbQuestion::fetch_by_topic(&self.slug)
    }

    pub fn get_topic_by_slug(slug: &str) -> DBResult<Self> {
        run_sync(async {
            let conn = get_conn();
            let mut rows = conn
                .query("SELECT slug FROM topics WHERE slug = ?1;", params![slug])
                .await?;
            if let Some(row) = rows.next().await? {
                let slug: String = row.get(0)?;
                Ok(DbTopic::new(&slug))
            } else {
                Err(DbErr::TopicsNotFoundInDb(slug.to_string()))
            }
        })
    }
}
