use serde;

use serde::de::{Deserialize, Deserializer};

use crate::deserializers::run_submit::StatusMessage;

pub(crate) fn int_from_bool<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    match bool::deserialize(deserializer)? {
        false => Ok(Some(0)),
        true => Ok(Some(1)),
    }
}

pub(crate) fn status_from_id<'de, D>(deserializer: D) -> Result<Option<StatusMessage>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<u32>::deserialize(deserializer)?.map(StatusMessage::from_status_code))
}
