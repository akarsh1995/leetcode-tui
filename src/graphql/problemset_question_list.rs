use serde::Serialize;
use serde_json::{json, Value};

use super::GQLLeetcodeQuery;

const QUERY: &str = r#"
query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
    problemsetQuestionList: questionList(
        categorySlug: $categorySlug
        limit: $limit
        skip: $skip
        filters: $filters
    ) {
        total: totalNum
        questions: data {
            acRate
            difficulty
            freqBar
            frontendQuestionId: questionFrontendId
            isFavor
            paidOnly: isPaidOnly
            status
            title
            titleSlug
            topicTags {
                name
                id
                slug
            }
            hasSolution
            hasVideoSolution
        }
    }
}"#;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Filters(Value);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Variables {
    category_slug: String,
    limit: i32,
    skip: i32,
    filters: Filters,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            category_slug: "".to_string(),
            limit: 1,
            skip: 0,
            filters: Filters(json!({})),
        }
    }
}

impl Variables {
    pub fn new(limit: i32, skip: i32) -> Self {
        Self {
            limit,
            skip,
            ..Default::default()
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    query: &'static str,
    variables: Variables,
}

impl Query {
    pub fn new(limit: i32, skip: i32) -> Self {
        Self {
            variables: Variables::new(limit, skip),
            ..Default::default()
        }
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            query: QUERY,
            variables: Variables::default(),
        }
    }
}
use crate::deserializers::problemset_question_list::Root;

impl GQLLeetcodeQuery for Query {
    type T = Root;
}
