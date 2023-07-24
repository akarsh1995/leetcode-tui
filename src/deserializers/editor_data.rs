use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CodeSnippet {
    lang: String,
    lang_slug: String,
    code: String,
}

#[derive(Debug, Deserialize, Serialize)]

#[serde(rename_all="camelCase")]
pub struct Question {
    question_id: String,
    question_frontend_id: String,
    code_snippets: Vec<CodeSnippet>,
    enable_run_code: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionEditorData {
    question: Question,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionData {
    data: QuestionEditorData,
}
