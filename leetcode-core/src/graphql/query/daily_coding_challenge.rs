use crate::types::daily_coding_challenge::IDailyCodingChallenge;

use super::GQLLeetcodeRequest;
use serde::Serialize;

const QUERY: &str = r#"
query questionOfToday {
  activeDailyCodingChallengeQuestion {
    date
    userStatus
    link
    question {
      acRate
      difficulty
      freqBar
      frontendQuestionId: questionFrontendId
      isFavor
      paidOnly: isPaidOnly
      status
      title
      titleSlug
      hasVideoSolution
      hasSolution
      topicTags {
        name
        id
        slug
      }
    }
  }
}"#;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    query: &'static str,
}

impl Query {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for Query {
    fn default() -> Self {
        Self { query: QUERY }
    }
}

impl GQLLeetcodeRequest for Query {
    type T = IDailyCodingChallenge;
}
