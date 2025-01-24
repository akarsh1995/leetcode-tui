use leetcode_core::types::problemset_question_list::Root;
use leetcode_tui_db::models::{question::DbQuestion, topic::DbTopic};

static JSON: &'static str = r#"{
            "data": {
                "problemsetQuestionList": {
                    "total": 2777,
                    "questions": [
                        {
                            "acRate": 45.35065222510613,
                            "difficulty": "Medium",
                            "freqBar": null,
                            "frontendQuestionId": "1",
                            "isFavor": false,
                            "paidOnly": false,
                            "status": "notac",
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
                        },
                        {
                            "acRate": 45.35065222510613,
                            "difficulty": "Medium",
                            "freqBar": null,
                            "frontendQuestionId": "2",
                            "isFavor": false,
                            "paidOnly": false,
                            "status": null,
                            "title": "Zigzag Conversion2",
                            "titleSlug": "zigzag-conversion2",
                            "topicTags": [
                                {
                                    "name": "String",
                                    "id": "VG9waWNUYWdOb2RlOjEw",
                                    "slug": "string"
                                }
                            ],
                            "hasSolution": true,
                            "hasVideoSolution": false
                        },
                        {
                            "acRate": 45.35065222510613,
                            "difficulty": "Medium",
                            "freqBar": null,
                            "frontendQuestionId": "3",
                            "isFavor": false,
                            "paidOnly": false,
                            "status": null,
                            "title": "Zigzag Conversion2",
                            "titleSlug": "zigzag-conversion2",
                            "topicTags": [
                                {
                                    "name": "Trees",
                                    "id": "VG9waWNUdOb2RlOjEw",
                                    "slug": "trees"
                                }
                            ],
                            "hasSolution": true,
                            "hasVideoSolution": false
                        },
                        {
                            "acRate": 48.35065222510613,
                            "difficulty": "Medium",
                            "freqBar": null,
                            "frontendQuestionId": "4",
                            "isFavor": false,
                            "paidOnly": false,
                            "status": null,
                            "title": "Zigzag Conversion2",
                            "titleSlug": "zigzag-conversion2",
                            "topicTags": [
                                {
                                    "name": "Trees",
                                    "id": "VG9waWNUdOb2RlOjEw",
                                    "slug": "trees"
                                }
                            ],
                            "hasSolution": true,
                            "hasVideoSolution": false
                        }                    
                    ]
                }
            }
        }"#;

fn populate_db<'a>() {
    let root: Root = serde_json::from_str(JSON).unwrap();
    let mut questions = root.get_questions();
    while let Some(quest) = questions.pop() {
        DbQuestion::try_from(quest).unwrap().save_to_db().unwrap();
    }
}

#[test]
fn test_should_fetch_all_topics_from_the_db() {
    leetcode_tui_db::init(None);
    populate_db();
    let topics = DbTopic::fetch_all().unwrap();
    assert_eq!(topics.len(), 2);
}

#[test]
fn test_should_fetch_all_questions_for_a_topic() {
    // Initialize the model
    leetcode_tui_db::init(None);

    populate_db();

    let topics = DbTopic::fetch_all().unwrap();

    let qs = topics[0].fetch_questions().unwrap();

    assert_eq!(qs.len(), 2);

    let qs = topics[1].fetch_questions().unwrap();

    assert_eq!(qs.len(), 2);
}
