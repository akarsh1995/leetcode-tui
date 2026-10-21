use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbErr {
    #[error("LibSqlError encountered: {0}")]
    LibSqlError(#[from] libsql::Error),

    #[error("Could not create topic {0}")]
    TopicCreateError(String),

    #[error("FrontEndQuestionIdParseError: {0}")]
    FrontEndQuestionIdParseError(#[from] ParseIntError),

    #[error("Question not found: {0}")]
    QuestionsNotFoundInDb(String),

    #[error("Topic not found: {0}")]
    TopicsNotFoundInDb(String),
}

pub type DBResult<T> = Result<T, DbErr>;
