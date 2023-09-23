use serde::{Deserialize, Serialize};

use super::language::Language;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSnippet {
    pub lang: String,
    pub lang_slug: Language,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question_id: String,
    pub question_frontend_id: String,
    pub code_snippets: Vec<CodeSnippet>,
    pub title_slug: String,
    pub enable_run_code: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionEditorData {
    pub question: Question,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionData {
    pub data: QuestionEditorData,
}
