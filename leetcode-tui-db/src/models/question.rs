use super::topic::DbTopic;
use crate::{
    api::types::problemset_question_list::Question,
    api::types::rest_problems_all::Problem,
    enums::Difficulty,
    errors::{DBResult, DbErr},
    get_conn, run_sync,
};
use libsql::params;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DbQuestion {
    pub id: u32,
    pub title: String,
    pub title_slug: String,
    pub difficulty: Difficulty,
    pub paid_only: bool,
    pub status: Option<String>,
    pub topics: Vec<DbTopic>,
    pub likes: u32,
    pub dislikes: u32,
    pub acceptance_rate: Option<f64>,
}

impl Ord for DbQuestion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DbQuestion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for DbQuestion {}

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
        write!(f, "{: >4}{w}", self.id)
    }
}

impl TryFrom<Question> for DbQuestion {
    type Error = DbErr;

    fn try_from(
        value: crate::api::types::problemset_question_list::Question,
    ) -> Result<Self, Self::Error> {
        let mut db_quest = DbQuestion::new(
            value.frontend_question_id.parse()?,
            value.title.as_str(),
            value.title_slug.as_str(),
            value.difficulty,
            value.paid_only,
            value.status,
            value.likes,
            value.dislikes,
            value.ac_rate,
        );
        if let Some(tts) = value.topic_tags {
            if !tts.is_empty() {
                for topic in tts {
                    db_quest.add_topic(topic.slug.as_str());
                }
            } else {
                db_quest.add_topic("unknown");
            }
        }
        Ok(db_quest)
    }
}

impl DbQuestion {
    pub fn new(
        id: u32,
        title: &str,
        title_slug: &str,
        difficulty: String,
        paid_only: bool,
        status: Option<String>,
        likes: u32,
        dislikes: u32,
        acceptance_rate: Option<f64>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            title_slug: title_slug.into(),
            topics: Default::default(),
            difficulty: match difficulty.as_str() {
                "Hard" => Difficulty::Hard,
                "Medium" => Difficulty::Medium,
                "Easy" => Difficulty::Easy,
                _ => panic!("Cannot parse difficulty"),
            },
            paid_only,
            status,
            likes,
            dislikes,
            acceptance_rate,
        }
    }

    pub fn try_from_rest_problem(problem: &Problem) -> Result<Self, DbErr> {
        let acceptance_rate = if problem.stat.total_submitted > 0 {
            Some((problem.stat.total_acs as f64 / problem.stat.total_submitted as f64) * 100.0)
        } else {
            None
        };
        let mut db_quest = DbQuestion::new(
            problem.stat.frontend_question_id,
            &problem.stat.title,
            &problem.stat.title_slug,
            problem.difficulty_str(),
            problem.paid_only,
            problem.status.clone(),
            0,
            0,
            acceptance_rate,
        );
        db_quest.add_topic("unknown");
        Ok(db_quest)
    }

    fn add_topic(&mut self, slug: &str) {
        self.topics.push(DbTopic::new(slug))
    }

    pub fn mark_accepted(&mut self) -> DBResult<Option<Vec<Self>>> {
        if self.status.is_none() || self.status == Some("notac".into()) {
            self.status = Some("ac".into());
            self.update_status_in_db()?;
            return Ok(Some(vec![self.clone()]));
        }
        Ok(None)
    }

    pub fn mark_attempted(&mut self) -> DBResult<Option<Vec<Self>>> {
        if self.status.is_none() {
            self.status = Some("notac".into());
            self.update_status_in_db()?;
            return Ok(Some(vec![self.clone()]));
        }
        Ok(None)
    }

    fn update_status_in_db(&self) -> DBResult<()> {
        run_sync(async {
            let conn = get_conn();
            conn.execute(
                "UPDATE questions SET status = ?1 WHERE id = ?2;",
                params![self.status.clone(), self.id as i64],
            )
            .await?;
            Ok(())
        })
    }

    pub fn get_total_questions() -> DBResult<usize> {
        run_sync(async {
            let conn = get_conn();
            let mut rows = conn.query("SELECT COUNT(*) FROM questions;", ()).await?;
            if let Some(row) = rows.next().await? {
                let count: i64 = row.get(0)?;
                Ok(count as usize)
            } else {
                Ok(0)
            }
        })
    }

    pub fn get_question_by_id(id: u32) -> DBResult<Option<Self>> {
        run_sync(async {
            let conn = get_conn();
            let mut rows = conn
                .query(
                    "SELECT q.id, q.title, q.title_slug, q.difficulty, q.paid_only, q.status, q.likes, q.dislikes, q.acceptance_rate, \
                     (SELECT GROUP_CONCAT(topic_slug, ',') FROM question_topics WHERE question_id = q.id) AS topic_slugs \
                     FROM questions q WHERE q.id = ?1;",
                    params![id as i64],
                )
                .await?;

            if let Some(row) = rows.next().await? {
                Ok(Some(Self::from_row(&row)?))
            } else {
                Ok(None)
            }
        })
    }

    pub fn get_topics(&self) -> &[DbTopic] {
        &self.topics
    }

    pub fn fetch_by_topic(topic_slug: &str) -> DBResult<Vec<Self>> {
        run_sync(async {
            let conn = get_conn();
            let mut rows = if topic_slug == "all" {
                conn.query(
                    "SELECT q.id, q.title, q.title_slug, q.difficulty, q.paid_only, q.status, q.likes, q.dislikes, q.acceptance_rate, \
                     (SELECT GROUP_CONCAT(topic_slug, ',') FROM question_topics WHERE question_id = q.id) AS topic_slugs \
                     FROM questions q ORDER BY q.id ASC;",
                    (),
                )
                .await?
            } else {
                conn.query(
                    "SELECT q.id, q.title, q.title_slug, q.difficulty, q.paid_only, q.status, q.likes, q.dislikes, q.acceptance_rate, \
                     (SELECT GROUP_CONCAT(topic_slug, ',') FROM question_topics WHERE question_id = q.id) AS topic_slugs \
                     FROM questions q \
                     JOIN question_topics qt ON q.id = qt.question_id \
                     WHERE qt.topic_slug = ?1 \
                     ORDER BY q.id ASC;",
                    params![topic_slug],
                )
                .await?
            };

            let mut questions = Vec::new();
            while let Some(row) = rows.next().await? {
                questions.push(Self::from_row(&row)?);
            }
            Ok(questions)
        })
    }

    fn from_row(row: &libsql::Row) -> DBResult<Self> {
        let id: i64 = row.get(0)?;
        let title: String = row.get(1)?;
        let title_slug: String = row.get(2)?;
        let difficulty_str: String = row.get(3)?;
        let paid_only: i64 = row.get(4)?;
        let status: Option<String> = row.get(5)?;
        let likes: i64 = row.get(6)?;
        let dislikes: i64 = row.get(7)?;
        let acceptance_rate: Option<f64> = row.get(8)?;
        let topic_slugs: Option<String> = row.get(9)?;

        let topics = topic_slugs
            .map(|s| {
                s.split(',')
                    .filter(|t| !t.is_empty())
                    .map(DbTopic::new)
                    .collect()
            })
            .unwrap_or_default();

        let difficulty = match difficulty_str.as_str() {
            "Hard" => Difficulty::Hard,
            "Medium" => Difficulty::Medium,
            "Easy" => Difficulty::Easy,
            _ => Difficulty::Easy,
        };

        Ok(DbQuestion {
            id: id as u32,
            title,
            title_slug,
            difficulty,
            paid_only: paid_only != 0,
            status,
            topics,
            likes: likes as u32,
            dislikes: dislikes as u32,
            acceptance_rate,
        })
    }

    pub fn save_to_db(&mut self) -> DBResult<bool> {
        run_sync(async {
            let conn = get_conn();
            conn.execute(
                "INSERT INTO questions (id, title, title_slug, difficulty, paid_only, status, likes, dislikes, acceptance_rate) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) \
                 ON CONFLICT(id) DO UPDATE SET \
                     title = excluded.title, \
                     title_slug = excluded.title_slug, \
                     difficulty = excluded.difficulty, \
                     paid_only = excluded.paid_only, \
                     status = excluded.status, \
                     likes = excluded.likes, \
                     dislikes = excluded.dislikes, \
                     acceptance_rate = excluded.acceptance_rate;",
                params![
                    self.id as i64,
                    self.title.clone(),
                    self.title_slug.clone(),
                    self.difficulty.to_string(),
                    if self.paid_only { 1i64 } else { 0i64 },
                    self.status.clone(),
                    self.likes as i64,
                    self.dislikes as i64,
                    self.acceptance_rate,
                ],
            )
            .await?;

            for topic in &self.topics {
                conn.execute(
                    "INSERT OR IGNORE INTO topics (slug) VALUES (?1);",
                    params![topic.slug.clone()],
                )
                .await?;
                conn.execute(
                    "INSERT OR IGNORE INTO question_topics (question_id, topic_slug) VALUES (?1, ?2);",
                    params![self.id as i64, topic.slug.clone()],
                )
                .await?;
            }

            Ok(true)
        })
    }

    pub fn save_multiple_to_db(questions: Vec<Self>) {
        run_sync(async {
            let conn = get_conn();
            conn.execute("BEGIN TRANSACTION;", ())
                .await
                .expect("Failed to begin transaction");

            for q in questions {
                conn.execute(
                    "INSERT INTO questions (id, title, title_slug, difficulty, paid_only, status, likes, dislikes, acceptance_rate) \
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) \
                     ON CONFLICT(id) DO UPDATE SET \
                         title = excluded.title, \
                         title_slug = excluded.title_slug, \
                         difficulty = excluded.difficulty, \
                         paid_only = excluded.paid_only, \
                         status = excluded.status, \
                         likes = excluded.likes, \
                         dislikes = excluded.dislikes, \
                         acceptance_rate = excluded.acceptance_rate;",
                    params![
                        q.id as i64,
                        q.title.clone(),
                        q.title_slug.clone(),
                        q.difficulty.to_string(),
                        if q.paid_only { 1i64 } else { 0i64 },
                        q.status.clone(),
                        q.likes as i64,
                        q.dislikes as i64,
                        q.acceptance_rate,
                    ],
                )
                .await
                .expect("Failed to insert question");

                for topic in &q.topics {
                    conn.execute(
                        "INSERT OR IGNORE INTO topics (slug) VALUES (?1);",
                        params![topic.slug.clone()],
                    )
                    .await
                    .expect("Failed to insert topic");
                    conn.execute(
                        "INSERT OR IGNORE INTO question_topics (question_id, topic_slug) VALUES (?1, ?2);",
                        params![q.id as i64, topic.slug.clone()],
                    )
                    .await
                    .expect("Failed to insert question_topic");
                }
            }

            conn.execute("COMMIT;", ())
                .await
                .expect("Failed to commit transaction");
        });
    }
}
