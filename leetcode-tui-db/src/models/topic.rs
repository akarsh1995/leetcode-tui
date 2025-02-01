use crate::{errors::DBResult, get_db_client};

use super::{question::DbQuestion, *};

#[native_model(id = 2, version = 1)]
#[native_db]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct DbTopic {
    #[primary_key]
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

    pub fn fetch_all<'a>() -> DBResult<Vec<DbTopic>> {
        let r = get_db_client().r_transaction()?;
        let x = r.scan().primary::<Self>()?.all().into_iter().collect();
        Ok(x)
    }

    pub fn fetch_questions<'a>(&self) -> DBResult<Vec<DbQuestion>> {
        let q_ids = if self.slug.eq("all") {
            (1..=5000).map(|x| x as u32).collect()
        } else {
            TopicQuestionMap::get_all_question_by_topic(self)?
        };
        let mut v = vec![];
        for q_id in q_ids {
            if let Some(available_ques) = DbQuestion::get_question_by_id(q_id)? {
                v.push(available_ques);
            };
        }
        Ok(v)
    }

    pub fn get_topic_by_slug<'a>(slug: &str) -> DBResult<Self> {
        let r = get_db_client().r_transaction()?;

        Ok(r.get()
            .primary(slug.to_string())?
            .ok_or(crate::errors::DbErr::TopicsNotFoundInDb(slug.to_string()))?)
    }
}
