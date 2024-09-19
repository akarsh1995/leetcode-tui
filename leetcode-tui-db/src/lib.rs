pub mod errors;
pub mod models;
use leetcode_core as api;
pub use models::{question::DbQuestion, topic::DbTopic};
use models::{QuestionTopicMap, TopicQuestionMap};
use native_db::DatabaseBuilder;
use leetcode_tui_shared::RoCell;

pub static DB_BUILDER: RoCell<DatabaseBuilder> = RoCell::new();

pub fn define_schema(db_builder: &mut DatabaseBuilder) -> errors::DBResult<()> {
    db_builder.define::<DbQuestion>()?;
    db_builder.define::<DbTopic>()?;
    db_builder.define::<QuestionTopicMap>()?;
    db_builder.define::<TopicQuestionMap>()?;
    Ok(())
}

pub fn init() {
    DB_BUILDER.init({
        let mut db_builder = DatabaseBuilder::new();
        define_schema(&mut db_builder).expect("DB schema initialization failed.");
        db_builder
    })
}
