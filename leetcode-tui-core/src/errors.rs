use std::{num::ParseIntError, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("Filename format does not match: {0}")]
    FileNameFormatDoesNotMatch(PathBuf),

    #[error("Couldn't parse language id: {0}")]
    LangIdParseError(#[from] ParseIntError),

    #[error("Filename does not exist for path: {0}")]
    FileNameDoesNotExistError(PathBuf),

    #[error("File name is not a valid utf8: {0}")]
    Utf8ValidityError(PathBuf),

    #[error("QuestionId: {0} does not exist")]
    QuestionIdDoesNotExist(String),
}

pub type CoreResult<T> = Result<T, CoreError>;
