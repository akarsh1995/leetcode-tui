use super::GQLLeetcodeQuery;
use serde::Serialize;

const QUERY: &str = r#"
query questionContent($titleSlug: String!) {
  question(titleSlug: $titleSlug) {
    content
    titleSlug
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

impl GQLLeetcodeQuery for Query {
    type T = crate::deserializers::question_content::Data;
}
