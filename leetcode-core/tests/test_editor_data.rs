use leetcode_core::types::editor_data::QuestionData;

#[test]
fn test_parse_editor_data() {
    let qdata: QuestionData =
        serde_json::from_str(include_str!("./test_editor_data.json")).unwrap();
    let res = qdata.get_editor_data_by_language(&leetcode_core::types::language::Language::Python3);
    assert_eq!(res.map(|r| r.starts_with("class")), Some(true));
    assert_eq!(res.map(|r| r.contains("List[int]")), Some(true));
    assert_eq!(res.map(|r| r.contains("dfjfjkdj")), Some(false))
}
