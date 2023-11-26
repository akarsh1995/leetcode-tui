use leetcode_core::types::run_submit_response::{ParsedResponse, RunSubmitResult};
use serde_json::{self, Value};

const JSONS_STR: &str = include_str!("./test_solution_run_parsing.json");

pub(crate) fn get_parsed_response(key_name: &str) -> ParsedResponse {
    let parsed: Value = serde_json::from_str(JSONS_STR).unwrap();
    let run_wrong_value = &parsed[key_name];
    let parsed_response: RunSubmitResult = serde_json::from_value(run_wrong_value.clone()).unwrap();
    parsed_response.to_parsed_response().unwrap()
}

#[test]
fn test_should_parse_run_correct_response_correctly() {
    assert!(matches!(
        get_parsed_response("run_correct"),
        ParsedResponse::RunAccepted { .. }
    ))
}

#[test]
fn test_should_parse_run_wrong_response_correctly() {
    assert!(matches!(
        get_parsed_response("run_wrong"),
        ParsedResponse::RunWrongAnswer { .. }
    ))
}

#[test]
fn test_should_parse_submit_correct_response_correctly() {
    assert!(matches!(
        get_parsed_response("submit_correct"),
        ParsedResponse::SubmitAccepted { .. }
    ))
}

#[test]
fn test_should_parse_submit_wrong_response_correctly() {
    assert!(matches!(
        get_parsed_response("submit_wrong"),
        ParsedResponse::SubmitWrongAnswer { .. }
    ))
}

#[test]
fn test_run_correct_response_output() {
    let parsed_response = get_parsed_response("run_correct");
    assert_eq!(
        parsed_response.to_string(),
        vec![
            "Solution ran successfully for 2/2 cases.",
            "Memory used: 2.00 MB",
            "Solution runtime: 0 ms",
        ]
        .join("\n")
    )
}

#[test]
fn test_run_wrong_response_output() {
    let parsed_response = get_parsed_response("run_wrong");
    assert_eq!(
        parsed_response.to_string(),
        vec![
            "Test Run Failed: 0/3 cases passed.",
            "Memory used: 16.39 MB",
            "Solution runtime: 82 ms",
        ]
        .join("\n")
    )
}

#[test]
fn test_submit_correct_response_output() {
    let parsed_response = get_parsed_response("submit_correct");
    assert_eq!(
        parsed_response.to_string(),
        vec![
            "Solution ran successfully for 57/57 cases.",
            "Memory used: 2.35 MB",
            "Solution runtime: 2 ms",
            "Your runtime beats 83.9281 % of the submissions.",
            "Your memory usage beats 39.7233 % of the submissions.",
        ]
        .join("\n")
    )
}

#[test]
fn test_submit_wrong_response_output() {
    let parsed_response = get_parsed_response("submit_wrong");
    assert_eq!(
        parsed_response.to_string(),
        vec![
            "3/80 cases passed.",
            "Memory used: 2.32 MB",
            "Solution runtime: N/A",
        ]
        .join("\n")
    )
}

#[test]
fn test_memory_limit_exceeded_response_output() {
    let parsed_response = get_parsed_response("memory_limit_exceeded");
    assert_eq!(
        parsed_response.to_string(),
        vec!["Memory Limit Exceeded: 976.69 MB"].join("\n")
    )
}
#[test]
fn test_output_limit_response_output() {
    let parsed_response = get_parsed_response("output_limit");
    assert_eq!(
        parsed_response.to_string(),
        vec![
            r#"Output Limit Exceeded: Last Testcase: "maybe long testcase""#,
            r#"Expected Output:"true""#,
            r#"Std Output: "some_long_string""#,
            r#"Code Output: """#,
        ]
        .join("\n")
    )
}
