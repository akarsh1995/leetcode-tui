pub mod errors;

use errors::{DBResult, DbErr};
use leetcode_core as api;

use api::types::problemset_question_list::Question;
use serde::{Deserialize, Serialize};
pub use surrealdb::engine::any::connect;
use surrealdb::engine::any::Any;
pub use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub type Db = Surreal<Any>;

impl TryFrom<Question> for DbQuestion {
    type Error = DbErr;

    fn try_from(
        value: api::types::problemset_question_list::Question,
    ) -> Result<Self, Self::Error> {
        let mut db_quest = DbQuestion::new(
            value.frontend_question_id.parse()?,
            value.title.as_str(),
            value.title_slug.as_str(),
        );
        if let Some(tts) = value.topic_tags {
            for topic in tts {
                db_quest.add_topic(topic.id.as_str(), topic.slug.as_str());
            }
        } else {
            db_quest.add_topic("unknown", "unknown");
        }
        Ok(db_quest)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct DbTopic {
    pub id: Thing,
    pub slug: String,
}

impl DbTopic {
    fn new(id: &str, slug: &str) -> Self {
        Self {
            id: ("topic", id).into(),
            slug: slug.into(),
        }
    }

    pub async fn fetch_all(db: &Db) -> DBResult<Vec<DbTopic>> {
        Ok(db.select("topic").await?)
    }

    pub async fn fetch_questions(&self, db: &Db) -> DBResult<Vec<DbQuestion>> {
        let mut k: Vec<DbQuestions> = db
            .query(format!(
                "select <-belongs_to_topic<-question as questions from {} fetch questions",
                self.id
            ))
            .await?
            .take(0)?;
        Ok(k.pop().map_or(vec![], |v| v.questions))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct DbQuestion {
    pub id: Thing,
    pub title: String,
    pub title_slug: String,
    pub topics: Vec<DbTopic>,
}

impl DbQuestion {
    fn new(id: i64, title: &str, title_slug: &str) -> Self {
        Self {
            id: Thing {
                tb: "question".into(),
                id: surrealdb::sql::Id::Number(id),
            },
            title: title.into(),
            title_slug: title_slug.into(),
            topics: Default::default(),
        }
    }

    fn add_topic(&mut self, id: &str, slug: &str) {
        self.topics.push(DbTopic::new(id, slug))
    }

    pub async fn update_in_db(&self, db: &Db) -> DBResult<Vec<Self>> {
        let q: Vec<Self> = db.update("question").content(self).await?;
        // TODO: topics may change update them
        Ok(q)
    }
}

impl DbQuestion {
    pub async fn to_db(&self, db: &Db) -> DBResult<Self> {
        let mut result: Vec<Self> = vec![];
        let written_questions_result = db.create("question").content(self).await;
        if let Err(e) = written_questions_result {
            match e {
                surrealdb::Error::Db(d) => match d {
                    surrealdb::error::Db::RecordExists { .. } => {
                        dbg!(d);
                    }
                    _ => return Err(DbErr::TopicCreateError(format!("{self:?} {d:?}"))),
                },
                surrealdb::Error::Api(ae) => match ae {
                    surrealdb::error::Api::Query(qe) => {
                        dbg!(qe);
                    }
                    _ => return Err(DbErr::TopicCreateError(format!("{self:?} {ae:?}"))),
                },
            }
        } else {
            let mut written_questions = written_questions_result.unwrap();
            result.append(&mut written_questions);

            for topic in &self.topics {
                let topic_create_res: Result<Vec<DbTopic>, surrealdb::Error> =
                    db.create("topic").content(topic).await;
                if let Err(e) = topic_create_res {
                    match e {
                        surrealdb::Error::Db(d) => match d {
                            surrealdb::error::Db::RecordExists { .. } => {
                                dbg!(d);
                            }
                            _ => return Err(DbErr::TopicCreateError(format!("{self:?} {d:?}"))),
                        },
                        surrealdb::Error::Api(ae) => match ae {
                            surrealdb::error::Api::Query(qe) => {
                                dbg!(qe);
                            }
                            _ => return Err(DbErr::TopicCreateError(format!("{self:?} {ae:?}"))),
                        },
                    }
                }
            }

            for t in &self.topics {
                db.query(format!("RELATE {}->belongs_to_topic->{}", self.id, t.id))
                    .await?;
            }
            return Ok(result.pop().unwrap());
        }
        Ok(self.clone())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DbQuestions {
    questions: Vec<DbQuestion>,
}

#[cfg(test)]
mod test {

    use super::*;

    use surrealdb::engine::any::connect;
    static JSON: &'static str = r#"{
            "data": {
                "problemsetQuestionList": {
                    "total": 2777,
                    "questions": [
                        {
                            "acRate": 45.35065222510613,
                            "difficulty": "Medium",
                            "freqBar": null,
                            "frontendQuestionId": "6",
                            "isFavor": false,
                            "paidOnly": false,
                            "status": "ac",
                            "title": "Zigzag Conversion",
                            "titleSlug": "zigzag-conversion",
                            "topicTags": [
                                {
                                    "name": "String",
                                    "id": "VG9waWNUYWdOb2RlOjEw",
                                    "slug": "string"
                                }
                            ],
                            "hasSolution": true,
                            "hasVideoSolution": false
                        }
                    ]
                }
            }
        }"#;

    use api::types::problemset_question_list::Root;

    #[tokio::test]
    async fn test_insert_all() {
        let db = connect("mem://").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let root: Root = serde_json::from_str(JSON).unwrap();
        let j = root
            .get_questions()
            .into_iter()
            .map(|q| DbQuestion::try_from(q).unwrap())
            .collect::<Vec<_>>();
        let res = j[0].to_db(&db).await.unwrap();
        assert_eq!(res.title_slug, "zigzag-conversion");
    }

    #[tokio::test]
    async fn test_update_question_details() {
        let db = connect("mem://").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let root: Root = serde_json::from_str(JSON).unwrap();
        let j = root
            .get_questions()
            .into_iter()
            .map(|q| DbQuestion::try_from(q).unwrap())
            .collect::<Vec<_>>();
        let mut res = j[0].to_db(&db).await.unwrap();
        res.title_slug = "Helloworld".to_string();
        let updated = res.update_in_db(&db).await.unwrap();
        assert_eq!(updated[0].title_slug, "Helloworld");
    }
}
