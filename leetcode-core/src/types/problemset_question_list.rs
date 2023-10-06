use serde::Deserialize;
use serde::{self, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicTag {
    pub name: String,
    pub id: String,
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub ac_rate: Option<f64>,
    pub difficulty: String,
    pub freq_bar: Option<f64>,
    pub frontend_question_id: String,
    pub is_favor: Option<bool>,
    pub paid_only: bool,
    pub status: Option<String>,
    pub title: String,
    pub title_slug: String,
    pub has_solution: Option<bool>,
    pub has_video_solution: Option<bool>,
    pub topic_tags: Option<Vec<TopicTag>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemSetQuestionList {
    pub total: i32,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub problemset_question_list: ProblemSetQuestionList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    data: Data,
}

impl Root {
    pub fn get_questions(self) -> Vec<Question> {
        self.data.problemset_question_list.questions
    }

    pub fn get_total_questions(&self) -> i32 {
        self.data.problemset_question_list.total
    }
}

#[cfg(test)]
mod tests {

    use super::Root;
    use serde_json;

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

        let root: Root = serde_json::from_str(json).unwrap();

        // Validate the deserialized struct fields
        assert_eq!(root.data.problemset_question_list.total, 2777);
        assert_eq!(root.data.problemset_question_list.questions.len(), 1);

        let question = &root.data.problemset_question_list.questions[0];
        assert_eq!(question.ac_rate, Some(45.35065222510613));
        assert_eq!(question.difficulty, Some("Medium".to_string()));
        assert_eq!(question.freq_bar, None);
        assert_eq!(question.frontend_question_id, "6".to_string());
        assert_eq!(question.is_favor, Some(false));
        assert_eq!(question.paid_only, Some(false));
        assert_eq!(question.status, Some("ac".to_string()));
        assert_eq!(question.title, ("Zigzag Conversion".to_string()));
        assert_eq!(question.title_slug, ("zigzag-conversion".to_string()));

        if let Some(topic_tags) = &question.topic_tags {
            assert_eq!(topic_tags.len(), 1);
            let topic_tag = &topic_tags[0];
            assert_eq!(topic_tag.name, ("String".to_string()));
            assert_eq!(topic_tag.id, "VG9waWNUYWdOb2RlOjEw");
            assert_eq!(topic_tag.slug, ("string".to_string()));
        }

        assert_eq!(question.has_solution, Some(true));
        assert_eq!(question.has_video_solution, Some(false));
    }
}
