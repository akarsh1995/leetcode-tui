pub mod errors;
pub mod models;
use errors::DBResult;
use leetcode_core as api;
pub use models::{question::DbQuestion, topic::DbTopic};
use models::{QuestionTopicMap, TopicQuestionMap};
use native_db::Database;
use native_db::DatabaseBuilder;
use native_db::Input;
use std::path::PathBuf;
use std::sync::OnceLock;

pub type Db<'a> = Database<'a>;

pub static DB_BUILDER: OnceLock<DatabaseBuilder> = OnceLock::new();
pub static DB_CLIENT: OnceLock<Db> = OnceLock::new();

pub(crate) fn get_db_client() -> &'static Database<'static> {
    DB_CLIENT.get().expect("Database client is not initialized")
}

pub fn define_schema(db_builder: &mut DatabaseBuilder) -> errors::DBResult<&mut DatabaseBuilder> {
    db_builder.define::<DbQuestion>()?;
    db_builder.define::<DbTopic>()?;
    db_builder.define::<QuestionTopicMap>()?;
    db_builder.define::<TopicQuestionMap>()?;
    Ok(db_builder)
}

pub fn init(db_path: Option<&PathBuf>) {
    let mut database_builder = DatabaseBuilder::new();
    define_schema(&mut database_builder).expect("DB schema initialization failed.");

    DB_BUILDER.get_or_init(|| database_builder);

    DB_CLIENT.get_or_init(|| {
        if let Some(path) = db_path {
            DB_BUILDER
                .get()
                .unwrap()
                .create(path)
                .expect("Error while creating db conn.")
        } else {
            DB_BUILDER
                .get()
                .unwrap()
                .create_in_memory()
                .expect("Error while creating db conn in memory.")
        }
    });
}

fn save<'a, T: Input + Clone>(item: &T) -> DBResult<()> {
    let rw = get_db_client().rw_transaction()?;
    rw.insert(item.clone())?;
    rw.commit()?;
    Ok(())
}

fn save_multiple<'a, T: Input + Clone>(items: &Vec<T>) -> DBResult<()> {
    let rw = get_db_client().rw_transaction()?;
    for item in items {
        rw.insert(item.clone())?;
    }
    rw.commit()?;
    Ok(())
}
