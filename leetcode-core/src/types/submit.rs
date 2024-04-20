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

impl SubmitCodeRequest {
    pub fn new(lang: Language, question_id: String, typed_code: String, slug: String) -> Self {
        Self {
            lang,
            question_id,
            typed_code,
            slug,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubmitCodeIntermediateResponse {
    pub submission_id: u32,
}
