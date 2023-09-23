use super::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubmitCodeRequest {
    pub lang: Language,
    pub question_id: String,
    pub typed_code: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubmitCodeIntermediateResponse {
    pub submission_id: u32,
}
