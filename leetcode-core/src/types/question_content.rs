use html2md::parse_html;
#[derive(Debug, serde::Deserialize)]
pub struct QueryQuestionContent {
    pub question: QuestionContent,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionContent {
    pub content: String,
    pub title_slug: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Data {
    pub data: QueryQuestionContent,
}

impl QuestionContent {
    pub fn html_to_text(&self) -> String {
        let string = self.content.as_str();
        let s: String = parse_html(string);
        s.lines().collect::<Vec<&str>>().join("\n")
    }
}
