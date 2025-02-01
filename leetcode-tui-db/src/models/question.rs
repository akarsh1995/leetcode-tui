use super::{topic::DbTopic, *};
use crate::{
    api::types::problemset_question_list::Question,
    errors::{DBResult, DbErr},
    get_db_client, save, save_multiple,
};
use std::fmt::Display;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct DbQuestion {
    #[primary_key]
    pub id: u32,
    pub title: String,
    pub title_slug: String,
    pub difficulty: String,
    pub paid_only: bool,
    pub status: Option<String>,
    pub topics: Vec<DbTopic>,
}

impl Ord for DbQuestion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DbQuestion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
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

impl DbQuestion {
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
    ) -> Self {
        Self {
            id,
            title: title.into(),
            title_slug: title_slug.into(),
            topics: Default::default(),
            difficulty,
            paid_only,
            status,
        }
    }

    fn add_topic(&mut self, slug: &str) {
        self.topics.push(DbTopic::new(slug))
    }

    pub fn mark_accepted<'a>(&mut self) -> DBResult<Option<Vec<Self>>> {
        if self.status.is_none() || self.status == Some("notac".into()) {
            self.status = Some("ac".into());
            return Ok(Some(self.update_in_db()?));
        }
        Ok(None)
    }

    pub fn mark_attempted<'a>(&mut self) -> DBResult<Option<Vec<Self>>> {
        if self.status.is_none() {
            self.status = Some("notac".into());
            return Ok(Some(self.update_in_db()?));
        }
        Ok(None)
    }

    fn update_in_db<'a>(&self) -> DBResult<Vec<Self>> {
        let rw = get_db_client().rw_transaction()?;
        let old = Self::get_question_by_id(self.id)?;
        if let Some(old_q) = old {
            rw.update(old_q, self.clone())?;
            rw.commit()?;
        }
        Ok(vec![self.clone()])
    }

    pub fn get_total_questions<'a>() -> DBResult<usize> {
        let r = get_db_client().r_transaction()?;
        let x = r.scan().primary::<Self>()?;
        Ok(x.all().count())
    }

    pub fn get_question_by_id<'a>(id: u32) -> DBResult<Option<Self>> {
        let r = get_db_client().r_transaction()?;
        let x = r.get().primary::<DbQuestion>(id)?;
        // x.topics = x.fetch_all_topics(db)?;
        Ok(x)
    }

    fn save_all_topics<'a>(&mut self) -> DBResult<()> {
        save_multiple(&self.topics)
    }

    pub(crate) fn get_topics(&self) -> &Vec<DbTopic> {
        &self.topics
    }

    fn get_topic_question_mapping(&self) -> Vec<TopicQuestionMap> {
        self.get_topics()
            .iter()
            .map(|q| TopicQuestionMap::new(&q.slug, self.id))
            .collect::<Vec<_>>()
    }

    fn get_question_topic_mapping(&self) -> Vec<QuestionTopicMap> {
        self.get_topics()
            .iter()
            .map(|q| QuestionTopicMap::new(self.id, &q.slug))
            .collect::<Vec<_>>()
    }

    pub fn save_multiple_to_db(questions: Vec<Self>) {
        let topic_question_map = questions
            .iter()
            .map(|q| q.get_topic_question_mapping())
            .flatten()
            .collect::<Vec<_>>();

        let question_topic_map = questions
            .iter()
            .map(|q| q.get_question_topic_mapping())
            .flatten()
            .collect::<Vec<_>>();

        let topics = questions
            .iter()
            .map(|q| q.get_topics().iter().map(|t| t.clone()))
            .flatten()
            .collect::<Vec<_>>();

        save_multiple(&topic_question_map).unwrap();
        save_multiple(&question_topic_map).unwrap();
        save_multiple(&topics).unwrap();
        save_multiple(&questions).unwrap();
    }

    pub fn save_to_db<'a>(&mut self) -> DBResult<bool> {
        // save Topic -> Question Mapping
        TopicQuestionMap::save_mapping(self)?;

        // save Question -> Topic Mapping
        QuestionTopicMap::save_mapping(self)?;

        // save DbTopics for the question
        self.save_all_topics()?;

        // save question
        save(self)?;
        return Ok(true);
    }
}
