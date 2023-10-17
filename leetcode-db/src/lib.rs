pub mod errors;

use std::fmt::Display;

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
            value.difficulty,
            value.paid_only,
            value.status,
        );
        if let Some(tts) = value.topic_tags {
            if !tts.is_empty() {
                for topic in tts {
                    db_quest.add_topic(topic.id.as_str(), topic.slug.as_str());
                }
            } else {
                db_quest.add_topic("unknown", "unknown");
            }
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
                "select array::sort::asc(<-belongs_to_topic<-question) as questions from {} fetch questions",
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
    pub difficulty: String,
    pub paid_only: bool,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Count {
    pub count: usize,
}

impl Display for DbQuestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut w = String::new();
        w.push_str(if self.paid_only { "🔐" } else { "  " });
        w.push_str(if self.status.is_none() {
            "  "
        } else if self.status == Some("ac".into()) {
            "👑"
        } else {
            "🏃"
        });
        w.push_str(self.title.as_str());
        write!(f, "{: >4}{w}", self.id.id.to_raw())
    }
}

impl DbQuestion {
    fn new(
        id: i64,
        title: &str,
        title_slug: &str,
        difficulty: String,
        paid_only: bool,
        status: Option<String>,
    ) -> Self {
        Self {
            id: Thing {
                tb: "question".into(),
                id: surrealdb::sql::Id::Number(id),
            },
            title: title.into(),
            title_slug: title_slug.into(),
            topics: Default::default(),
            difficulty,
            paid_only,
            status,
        }
    }

    fn add_topic(&mut self, id: &str, slug: &str) {
        self.topics.push(DbTopic::new(id, slug))
    }

    pub async fn mark_accepted(&mut self, db: &Db) -> DBResult<Option<Vec<Self>>> {
        if self.status.is_none() || self.status == Some("notac".into()) {
            self.status = Some("ac".into());
            return Ok(Some(self.update_in_db(db).await?));
        }
        Ok(None)
    }

    pub async fn mark_attempted(&mut self, db: &Db) -> DBResult<Option<Vec<Self>>> {
        if self.status.is_none() {
            self.status = Some("notac".into());
            return Ok(Some(self.update_in_db(db).await?));
        }
        Ok(None)
    }

    pub async fn update_in_db(&self, db: &Db) -> DBResult<Vec<Self>> {
        let q: Vec<Self> = db.update("question").content(self).await?;
        // TODO: topics may change update them
        Ok(q)
    }

    pub async fn get_total_questions(db: &Db) -> DBResult<Count> {
        let mut k: Vec<Count> = db
            .query("SELECT count() FROM question GROUP ALL")
            .await?
            .take(0)?;
        if let Some(_count) = k.pop() {
            Ok(_count)
        } else {
            Err(DbErr::QuestionsNotFoundInDb("".into()))
        }
    }

    pub fn is_hard(&self) -> bool {
        self.difficulty == "Hard"
    }

    pub fn is_medium(&self) -> bool {
        self.difficulty == "Medium"
    }

    pub fn is_easy(&self) -> bool {
        self.difficulty == "Easy"
    }
}

impl DbQuestion {
    pub async fn to_db(&self, db: &Db) -> DBResult<Self> {
        let mut result: Vec<Self> = vec![];
        let written_questions_result = db.create("question").content(self).await;
        if let Err(e) = written_questions_result {
            match e {
                surrealdb::Error::Db(d) => match d {
                    surrealdb::error::Db::RecordExists { .. } => {}
                    _ => return Err(DbErr::TopicCreateError(format!("{self:?} {d:?}"))),
                },
                surrealdb::Error::Api(ae) => match ae {
                    surrealdb::error::Api::Query(_qe) => {}
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
                            surrealdb::error::Db::RecordExists { .. } => {}
                            _ => return Err(DbErr::TopicCreateError(format!("{self:?} {d:?}"))),
                        },
                        surrealdb::Error::Api(ae) => match ae {
                            surrealdb::error::Api::Query(_qe) => {}
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

    #[tokio::test]
    async fn test_mark_question_attempted() {
        let db = connect("mem://").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let root: Root = serde_json::from_str(JSON).unwrap();
        let mut q: DbQuestion = root.get_questions().pop().unwrap().try_into().unwrap();
        q.status = None;
        let mut res = q.to_db(&db).await.unwrap();
        assert_eq!(res.status, None);
        let attempted = res
            .mark_attempted(&db)
            .await
            .unwrap()
            .unwrap()
            .pop()
            .unwrap();
        assert_eq!(attempted.status, Some("notac".into()));
        let db_quest: DbQuestion = db.select(("question", 6)).await.unwrap().unwrap();
        assert_eq!(db_quest.status, Some("notac".into()));
    }

    #[tokio::test]
    async fn test_mark_question_accepted() {
        let db = connect("mem://").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let root: Root = serde_json::from_str(JSON).unwrap();
        let mut q: DbQuestion = root.get_questions().pop().unwrap().try_into().unwrap();
        q.status = None;
        let mut res = q.to_db(&db).await.unwrap();
        assert_eq!(res.status, None);
        let accepted = res
            .mark_accepted(&db)
            .await
            .unwrap()
            .unwrap()
            .pop()
            .unwrap();
        assert_eq!(accepted.status, Some("ac".into()));
        let db_quest: DbQuestion = db.select(("question", 6)).await.unwrap().unwrap();
        assert_eq!(db_quest.status, Some("ac".into()));
    }

    #[tokio::test]
    async fn test_mark_attempted_question_accepted() {
        let db = connect("mem://").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let root: Root = serde_json::from_str(JSON).unwrap();
        let mut q: DbQuestion = root.get_questions().pop().unwrap().try_into().unwrap();
        q.status = Some("notac".into());
        let mut res = q.to_db(&db).await.unwrap();
        assert_eq!(res.status, Some("notac".into()));
        let accepted = res
            .mark_accepted(&db)
            .await
            .unwrap()
            .unwrap()
            .pop()
            .unwrap();
        assert_eq!(accepted.status, Some("ac".into()));
        let db_quest: DbQuestion = db.select(("question", 6)).await.unwrap().unwrap();
        assert_eq!(db_quest.status, Some("ac".into()));
    }

    #[tokio::test]
    async fn test_try_mark_accepted_question_attempted() {
        let db = connect("mem://").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let root: Root = serde_json::from_str(JSON).unwrap();
        let q: DbQuestion = root.get_questions().pop().unwrap().try_into().unwrap();
        let mut res = q.to_db(&db).await.unwrap();
        assert_eq!(res.status, Some("ac".into()));
        let maybe_accepted = res.mark_attempted(&db).await.unwrap();
        assert_eq!(maybe_accepted, None);
        let db_quest: DbQuestion = db.select(("question", 6)).await.unwrap().unwrap();
        assert_eq!(db_quest.status, Some("ac".into()));
    }
}
