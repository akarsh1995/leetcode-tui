use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::errors::DBResult;

use self::topic::DbTopic;
pub mod question;
pub mod topic;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[native_model(id = 3, version = 1)]
#[native_db]
pub(crate) struct TopicQuestionMap {
    #[primary_key]
    id: String,
    #[secondary_key]
    topic_id: String,
    question_id: u32,
}

impl TopicQuestionMap {
    fn new(topic_id: &str, question_id: u32) -> Self {
        Self {
            id: format!("{topic_id}_{question_id}"),
            topic_id: topic_id.to_string(),
            question_id,
        }
    }

    pub(crate) fn save_mapping<'a>(question: &question::DbQuestion, db: &Database) -> DBResult<()> {
        for topic in question.get_topics() {
            let topic_question_mapping = Self::new(&topic.slug, question.id);
            topic_question_mapping.save_to_db(db)?;
        }
        Ok(())
    }

    fn save_to_db<'a>(&self, db: &Database) -> DBResult<()> {
        let rw_trans = db.rw_transaction()?;
        rw_trans.insert(self.clone())?;
        rw_trans.commit()?;
        Ok(())
    }

    pub(crate) fn get_all_question_by_topic(topic: &DbTopic, db: &Database) -> DBResult<Vec<u32>> {
        let trans = db.r_transaction()?;
        let mut quests = vec![];
        for tq_map in trans
            .scan()
            .secondary::<Self>(TopicQuestionMapKey::topic_id)?
            .start_with(topic.slug.to_string())
        {
            quests.push(tq_map.question_id);
        }
        Ok(quests)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[native_model(id = 4, version = 1)]
#[native_db]
pub(crate) struct QuestionTopicMap {
    #[primary_key]
    id: String,
    #[secondary_key]
    question_id: u32,
    topic_id: String,
}

impl QuestionTopicMap {
    fn new(question_id: u32, topic_id: &str) -> Self {
        Self {
            id: format!("{question_id}_{topic_id}"),
            question_id,
            topic_id: topic_id.to_string(),
        }
    }

    pub(crate) fn save_mapping<'a>(question: &question::DbQuestion, db: &Database) -> DBResult<()> {
        for topic in question.get_topics() {
            let question_topic_mapping = Self::new(question.id, &topic.slug);
            question_topic_mapping.save_to_db(db)?;
        }
        Ok(())
    }

    fn save_to_db<'a>(&self, db: &Database) -> DBResult<()> {
        let rw_trans = db.rw_transaction()?;
        rw_trans.insert(self.clone())?;
        rw_trans.commit()?;
        Ok(())
    }
}
