use leetcode_core::types::problemset_question_list::Root;
use leetcode_tui_db::models::question::DbQuestion;

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

fn populate_db<'a>() {
    let root: Root = serde_json::from_str(JSON).unwrap();
    let mut questions = root.get_questions();
    let mut db_quest1 = DbQuestion::try_from(questions.pop().unwrap()).unwrap();
    let mut db_quest2 = DbQuestion::try_from(questions.pop().unwrap()).unwrap();
    db_quest1.save_to_db().unwrap();
    db_quest2.save_to_db().unwrap();
}

#[test]
fn test_it_should_return_the_number_of_questions_correctly() {
    leetcode_tui_db::init(None);
    populate_db();

    let result = DbQuestion::get_total_questions().unwrap();
    assert_eq!(result, 2)
}

#[test]
fn test_it_should_mark_the_question_accepted_correctly() {
    leetcode_tui_db::init(None);
    populate_db();
    let mut x = DbQuestion::get_question_by_id(1).unwrap();
    x.mark_accepted().unwrap();

    let x = DbQuestion::get_question_by_id(1).unwrap();
    assert_eq!(x.status, Some("ac".into()));
}

#[test]
fn test_it_should_mark_the_question_attempted_correctly() {
    leetcode_tui_db::init(None);
    populate_db();
    let mut x = DbQuestion::get_question_by_id(2).unwrap();

    assert_eq!(x.status, None);

    x.mark_attempted().unwrap();

    let x = DbQuestion::get_question_by_id(2).unwrap();
    assert_eq!(x.status, Some("notac".into()));
}

#[test]
fn test_it_should_add_a_new_question_to_db() {
    leetcode_tui_db::init(None);
    populate_db();
    let mut x = DbQuestion::new(5, "helloworld", "helloworld", "medium".into(), true, None);
    x.save_to_db().unwrap();

    assert_eq!(DbQuestion::get_total_questions().unwrap(), 3);

    let x = DbQuestion::get_question_by_id(5).unwrap();
    assert_eq!(x.id, 5);
    assert_eq!(x.title, "helloworld");
}
