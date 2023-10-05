use super::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RunCodeRequest {
    pub lang: Language,
    pub question_id: String,
    pub typed_code: String,
    #[serde(rename = "data_input")]
    pub test_cases_stdin: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub slug: String,
}

impl RunCodeRequest {
    pub fn new(lang: Language, question_id: String, typed_code: String, slug: String) -> Self {
        Self {
            lang,
            question_id,
            typed_code,
            slug,
            test_cases_stdin: Some("".to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RunCodeIntermediateResponse {
    pub interpret_id: String,
    test_case: String,
}
