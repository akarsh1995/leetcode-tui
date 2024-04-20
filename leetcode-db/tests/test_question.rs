mod common;

use common::build_db;
use leetcode_core::types::problemset_question_list::Root;
use leetcode_db::models::question::DbQuestion;
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
                        }                    
                    ]
                }
            }
        }"#;

fn populate_db<'a>(db: &Database<'a>) {
    let root: Root = serde_json::from_str(JSON).unwrap();
    let mut questions = root.get_questions();
    let db_quest1 = DbQuestion::try_from(questions.pop().unwrap()).unwrap();
    let db_quest2 = DbQuestion::try_from(questions.pop().unwrap()).unwrap();
    let trans = db.rw_transaction().unwrap();
    trans.insert::<DbQuestion>(db_quest1).unwrap();
    trans.insert::<DbQuestion>(db_quest2).unwrap();
    trans.commit().unwrap();
}

#[test]
fn test_it_should_return_the_number_of_questions_correctly() {
    let mut db_builder = DatabaseBuilder::new();
    // Initialize the model

    let db = build_db(&mut db_builder).unwrap();
    populate_db(&db);

    let result = DbQuestion::get_total_questions(&db).unwrap();
    assert_eq!(result, 2)
}

#[test]
fn test_it_should_mark_the_question_accepted_correctly() {
    let mut db_builder = DatabaseBuilder::new();
    // Initialize the model

    let db = build_db(&mut db_builder).unwrap();
    populate_db(&db);

    let mut x = DbQuestion::get_question_by_id(&db, 1).unwrap();
    x.mark_accepted(&db).unwrap();

    let x = DbQuestion::get_question_by_id(&db, 1).unwrap();
    assert_eq!(x.status, Some("ac".into()));
}

#[test]
fn test_it_should_mark_the_question_attempted_correctly() {
    let mut db_builder = DatabaseBuilder::new();
    // Initialize the model

    let db = build_db(&mut db_builder).unwrap();
    populate_db(&db);

    let mut x = DbQuestion::get_question_by_id(&db, 2).unwrap();

    assert_eq!(x.status, None);

    x.mark_attempted(&db).unwrap();

    let x = DbQuestion::get_question_by_id(&db, 2).unwrap();
    assert_eq!(x.status, Some("notac".into()));
}

#[test]
fn test_it_should_add_a_new_question_to_db() {
    let mut db_builder = DatabaseBuilder::new();
    // Initialize the model

    let db = build_db(&mut db_builder).unwrap();
    populate_db(&db);

    let mut x = DbQuestion::new(5, "helloworld", "helloworld", "medium".into(), true, None);
    x.save_to_db(&db).unwrap();

    assert_eq!(DbQuestion::get_total_questions(&db).unwrap(), 3);

    let x = DbQuestion::get_question_by_id(&db, 5).unwrap();
    assert_eq!(x.id, 5);
    assert_eq!(x.title, "helloworld");
}
