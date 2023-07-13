use super::topic_tag;
use serde;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProblemSetQuestionList {
    total: i32,
    questions: Vec<crate::entities::question::Model>,
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
                            "acRate": 45.35065222510613,
                            "difficulty": "Medium",
                            "freqBar": null,
                            "frontendQuestionId": "6",
                            "isFavor": false,
                            "paidOnly": false,
                            "status": "ac",
                            "title": "Zigzag Conversion",
                            "titleSlug": "zigzag-conversion",
                            "topicTags": [
                                {
                                    "name": "String",
                                    "id": "VG9waWNUYWdOb2RlOjEw",
                                    "slug": "string"
                                }
                            ],
                            "hasSolution": true,
                            "hasVideoSolution": false
                        }
                    ]
                }
            }
        }"#;

        let root: ProblemSetQuestionListRoot = serde_json::from_str(json).unwrap();

        // Validate the deserialized struct fields
        assert_eq!(root.data.problemset_question_list.total, 2777);
        assert_eq!(root.data.problemset_question_list.questions.len(), 1);

        let question = &root.data.problemset_question_list.questions[0];
        assert_eq!(question.ac_rate, Some(45.35065222510613));
        assert_eq!(question.difficulty, Some("Medium".to_string()));
        assert_eq!(question.freq_bar, None);
        assert_eq!(question.frontend_question_id, "6".to_string());
        assert_eq!(question.is_favor, Some(0));
        assert_eq!(question.paid_only, Some(0));
        assert_eq!(question.status, Some("ac".to_string()));
        assert_eq!(question.title, Some("Zigzag Conversion".to_string()));
        assert_eq!(question.title_slug, Some("zigzag-conversion".into()));
        // assert_eq!(question.topic_tags.len(), 1);

        // let topic_tag = &question.topic_tags[0];
        // assert_eq!(topic_tag.name, "String");
        // assert_eq!(topic_tag.id, "VG9waWNUYWdOb2RlOjEw");
        // assert_eq!(topic_tag.slug, "string");

        assert_eq!(question.has_solution, Some(1));
        assert_eq!(question.has_video_solution, Some(0));
    }
}
