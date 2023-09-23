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

#[derive(Debug, Deserialize, Serialize)]
pub struct RunCodeIntermediateResponse {
    pub interpret_id: String,
    test_case: String,
}
