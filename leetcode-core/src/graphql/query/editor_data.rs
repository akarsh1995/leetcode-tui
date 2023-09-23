use super::GQLLeetcodeRequest;
use serde::Serialize;

const QUERY: &str = r#"
query questionEditorData($titleSlug: String!) {
  question(titleSlug: $titleSlug) {
    questionId
    titleSlug
    questionFrontendId
    codeSnippets {
      lang
      langSlug
      code
    }
    envInfo
    enableRunCode
  }
}
"#;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Variables {
    title_slug: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    query: &'static str,
    variables: Variables,
}

impl Query {
    pub fn new(title_slug: String) -> Self {
        Self {
            query: QUERY,
            variables: Variables { title_slug },
        }
    }
}

impl GQLLeetcodeRequest for Query {
    type T = crate::types::editor_data::QuestionData;
}
