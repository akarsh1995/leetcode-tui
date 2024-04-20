mod common;

use common::build_db;
use leetcode_core::types::problemset_question_list::Root;
use leetcode_tui_db::models::{question::DbQuestion, topic::DbTopic};
use native_db::{Database, DatabaseBuilder};

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

fn populate_db<'a>(db: &Database<'a>) {
    let root: Root = serde_json::from_str(JSON).unwrap();
    let mut questions = root.get_questions();
    while let Some(quest) = questions.pop() {
        DbQuestion::try_from(quest).unwrap().save_to_db(db).unwrap();
    }
}

#[test]
fn test_should_fetch_all_topics_from_the_db() {
    let mut db_builder = DatabaseBuilder::new();
    // Initialize the model
    let db = build_db(&mut db_builder).unwrap();

    populate_db(&db);
    let topics = DbTopic::fetch_all(&db).unwrap();
    assert_eq!(topics.len(), 2);
}

#[test]
fn test_should_fetch_all_questions_for_a_topic() {
    let mut db_builder = DatabaseBuilder::new();
    // Initialize the model
    let db = build_db(&mut db_builder).unwrap();

    populate_db(&db);

    let topics = DbTopic::fetch_all(&db).unwrap();

    let qs = topics[0].fetch_questions(&db).unwrap();

    assert_eq!(qs.len(), 2);

    let qs = topics[1].fetch_questions(&db).unwrap();

    assert_eq!(qs.len(), 2);
}
