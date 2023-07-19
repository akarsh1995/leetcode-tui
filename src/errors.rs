use sea_orm::error::DbErr;

use thiserror::Error;

use crate::app_ui::channel::*;
use crate::app_ui::event::Event;

#[derive(Error, Debug)]
pub enum LcAppError {
    #[error("Send error to reciever at sync context")]
    SyncSendError(#[from] std::sync::mpsc::SendError<Event>),

    #[error("Receive error in sync context")]
    SyncReceiveError(#[from] std::sync::mpsc::RecvError),

    #[error("Task request send error sync to async context: {0}")]
    RequestSendError(#[from] RequestSendError),

    #[error("Task request receive error sync to async context: {0}")]
    RequestRecvError(#[from] RequestRecvError),

    #[error("Task response send error async to sync context: {0}")]
    ResponseSendError(#[from] ResponseSendError),

    #[error("Task response receive error async to sync context: {0}")]
    ResponseReceiveError(#[from] ResponseReceiveError),

    #[error("Deserialization/serialization failed: {0}")]
    DeserializeError(#[from] serde_json::Error),

    #[error("Network request error.")]
    RequestError(#[from] reqwest::Error),

    #[error("IO Error")]
    IOError(#[from] std::io::Error),

    #[error("Crossterm Error {0}")]
    CrossTermError(String),

    #[error("Database Error encountered {0}")]
    DatabaseError(#[from] DbErr),

    #[error("Maybe could not find xdg dirs {0}")]
    XDGError(#[from] xdg::BaseDirectoriesError),

    #[error("Toml parsing error")]
    TOMLParseError(#[from] toml::de::Error),

    #[error("Toml serialization error")]
    TOMLSerializeError(#[from] toml::ser::Error),

    #[error("Error while building reqwest client: {0}")]
    ClientBuildError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Tokio join handle error")]
    TokioThreadJoinError(#[from] tokio::task::JoinError),

    // #[error("Crossterm Error")]
    // CrossTermError(#[from] crossterm::ErrorKind),

    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    #[error("unknown lc app error")]
    Unknown,
}

pub type AppResult<T> = Result<T, LcAppError>;
