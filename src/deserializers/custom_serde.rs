use serde;

use serde::de::{Deserialize, Deserializer};

pub(crate) fn int_from_bool<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    match bool::deserialize(deserializer)? {
        false => Ok(Some(0)),
        true => Ok(Some(1)),
    }
}
