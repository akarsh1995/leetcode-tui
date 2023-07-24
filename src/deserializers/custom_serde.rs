use serde;

use serde::de::{Deserialize, Deserializer};

use crate::graphql::check_run_submit::StatusMessage;

pub(crate) fn int_from_bool<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    match bool::deserialize(deserializer)? {
        false => Ok(Some(0)),
        true => Ok(Some(1)),
    }
}

pub(crate) fn status_from_string<'de, D>(deserializer: D) -> Result<StatusMessage, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match String::deserialize(deserializer)?.as_str() {
        "Runtime Error" => StatusMessage::RuntimeError,
        "Compile Error" => StatusMessage::CompileError,
        "Wrong Answer" => StatusMessage::WrongAnswer,
        "Accepted" => StatusMessage::Accepted,
        unknown => StatusMessage::Unknown(unknown.to_string()),
    })
}
