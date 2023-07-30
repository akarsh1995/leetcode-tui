use leetcode_tui_rs::deserializers::editor_data::QuestionData;

#[test]
fn test_parse_editor_data() {
    let qdata: QuestionData =
        serde_json::from_str(include_str!("./test_editor_data.json")).unwrap();
    dbg!(qdata);
}
