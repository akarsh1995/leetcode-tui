use html2text::from_read;
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

impl QuestionContent {
    pub fn html_to_text(&self) -> String {
        let string = self.content.as_bytes();
        let s: String = from_read(string, string.len());
        s.replace("\\n\\n", "\n\n")
            .lines()
            .filter(|l| *l != "")
            .collect::<Vec<&str>>()
            .join("\n")
    }
}
