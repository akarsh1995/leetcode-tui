pub mod event;
pub mod step;
pub mod topic;
use std::error::Error;

pub use event::Event;
pub mod popup;
pub mod question;
pub mod utils;

pub mod errors {
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
    }

    pub type CoreResult<T> = Result<T, CoreError>;
}

pub trait SendError<T, E> {
    fn emit(self) -> Result<T, E>;
}

impl<T, E> SendError<T, E> for Result<T, E>
where
    E: Error + Sized,
{
    fn emit(self) -> Result<T, E> {
        match self {
            Err(e) => {
                emit!(Error(e.to_string()));
                Err(e)
            }
            ok => ok,
        }
    }
}

pub fn init() {
    question::init();
}
