use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbErr {
    #[error("DBErr: {0}")]
    DbErr(#[from] surrealdb::Error),

    #[error("Could not create topic {0}")]
    TopicCreateError(String),

    #[error("FrontEndQuestionIdParseError: {0}")]
    FrontEndQuestionIdParseError(#[from] ParseIntError),

    #[error("Questions not found: {0}")]
    QuestionsNotFoundInDb(String),
}

pub type DBResult<T> = Result<T, DbErr>;