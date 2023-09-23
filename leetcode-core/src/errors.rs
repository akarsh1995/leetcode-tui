use thiserror::Error;

#[derive(Error, Debug)]
pub enum LcAppError {
    #[error("Looks like your cookies has been expired kindly update your cookies in config.toml")]
    CookiesExpiredError,

    #[error("Deserialization/serialization failed: {0}")]
    DeserializeError(#[from] serde_json::Error),

    #[error("Network request error.")]
    RequestError(#[from] reqwest::Error),

    #[error("Status {code:?}: {contents:?}")]
    StatusCodeError { code: String, contents: String },

    #[error("Error while building reqwest client: {0}")]
    ClientBuildError(#[from] reqwest::header::InvalidHeaderValue),
}

pub type AppResult<T> = Result<T, LcAppError>;
