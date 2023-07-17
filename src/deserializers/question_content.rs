#[derive(Debug, serde::Deserialize)]
pub struct QueryQuestionContent {
    pub question: QuestionContent,
}

#[derive(Debug, serde::Deserialize)]
pub struct QuestionContent {
    pub content: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Data {
    pub data: QueryQuestionContent,
}
