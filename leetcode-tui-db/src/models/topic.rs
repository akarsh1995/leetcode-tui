use crate::errors::DBResult;

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

    pub fn fetch_all<'a>(db: &'a Database<'a>) -> DBResult<Vec<DbTopic>> {
        let r = db.r_transaction()?;
        let x = r.scan().primary::<Self>()?.all().into_iter().collect();
        Ok(x)
    }

    pub fn fetch_questions<'a>(&self, db: &'a Database<'a>) -> DBResult<Vec<DbQuestion>> {
        let q_ids = TopicQuestionMap::get_all_question_by_topic(self, db)?;
        let mut v = vec![];
        for q_id in q_ids {
            let q = DbQuestion::get_question_by_id(db, q_id)?;
            v.push(q);
        }
        Ok(v)
    }

    pub fn get_topic_by_slug<'a>(slug: &str, db: &'a Database<'a>) -> DBResult<Self> {
        let r = db.r_transaction()?;

        Ok(r.get()
            .primary(slug.to_string())?
            .ok_or(crate::errors::DbErr::TopicsNotFoundInDb(slug.to_string()))?)
    }

    pub(crate) fn save_to_db<'a>(&self, db: &'a Database<'a>) -> DBResult<()> {
        let rw = db.rw_transaction()?;
        rw.insert(self.clone())?;
        rw.commit()?;
        Ok(())
    }
}
