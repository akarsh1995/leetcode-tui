use serde;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub topic_tags: Vec<crate::entities::topic_tag::Model>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProblemSetQuestionList {
    questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    problemset_question_list: ProblemSetQuestionList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemSetQuestionListRoot {
    data: Data,
}

impl ProblemSetQuestionListRoot {
    pub fn get_questions_with_topics(&mut self) -> &mut Vec<Question> {
        &mut self.data.problemset_question_list.questions
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;

    #[test]
    fn test_original() {
        let s = r#"{"data":{"problemsetQuestionList":{"total":2777,"questions":[{"acRate":50.194408705463644,"difficulty":"Easy","freqBar":null,"frontendQuestionId":"1","isFavor":false,"paidOnly":false,"status":null,"title":"Two Sum","titleSlug":"two-sum","topicTags":[{"name":"Array","id":"VG9waWNUYWdOb2RlOjU=","slug":"array"},{"name":"Hash Table","id":"VG9waWNUYWdOb2RlOjY=","slug":"hash-table"}],"hasSolution":true,"hasVideoSolution":true}]}}}"#;
        let root: ProblemSetQuestionListRoot = serde_json::from_str(s).unwrap();
    }

    #[test]
    fn test_json_deserialization() {
        let json = r#"{
            "data": {
                "problemsetQuestionList": {
                    "total": 2777,
                    "questions": [
                        {
                            "topicTags": [
                                {
                                    "name": "String",
                                    "id": "VG9waWNUYWdOb2RlOjEw",
                                    "slug": "string"
                                }
                            ]
                        }
                    ]
                }
            }
        }"#;

        let root: ProblemSetQuestionListRoot = serde_json::from_str(json).unwrap();

        // Validate the deserialized struct fields
        let question = &root.data.problemset_question_list.questions[0];

        assert_eq!(question.topic_tags.len(), 1);

        let topic_tag = &question.topic_tags[0];
        assert_eq!(topic_tag.id, "VG9waWNUYWdOb2RlOjEw");
        assert_eq!(topic_tag.name, Some("String".into()));
        assert_eq!(topic_tag.slug, Some("string".into()));
    }
}
