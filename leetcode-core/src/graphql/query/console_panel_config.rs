use super::GQLLeetcodeRequest;
use serde::Serialize;

const QUERY: &str = r#"
query consolePanelConfig($titleSlug: String!) {
  question(titleSlug: $titleSlug) {
    questionFrontendId
    questionTitle
    exampleTestcaseList
    # metaData
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
    type T = crate::types::console_panel_config::Root;

    fn use_cache(&self) -> bool {
        true
    }
}
