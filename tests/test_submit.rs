use leetcode_tui_rs::graphql::check_run_submit::{ParsedResponse, RunResponse};
use serde_json::{self, Value};

const JSONS_STR: &str = include_str!("./test_submit.json");

#[test]
fn test_run_status_parsing() {
    let run_responses: Value = serde_json::from_str(JSONS_STR).unwrap();
    let compile_error = &run_responses[&"compile_error"];
    let runtime_error = &run_responses[&"runtime_error"];
    let run_success = &run_responses[&"success"];
    let pending = &run_responses[&"pending"];
    let started = &run_responses[&"started"];
    let mem_limit = &run_responses[&"memory_limit_exceeded"];
    let out_limit = &run_responses[&"output_limit"];
    let submit_success = &run_responses[&"submit_successful"];
    let re: RunResponse = serde_json::from_value(runtime_error.to_owned()).unwrap();
    let ce: RunResponse = serde_json::from_value(compile_error.to_owned()).unwrap();
    let rs: RunResponse = serde_json::from_value(run_success.to_owned()).unwrap();
    let pending: RunResponse = serde_json::from_value(pending.to_owned()).unwrap();
    let started: RunResponse = serde_json::from_value(started.to_owned()).unwrap();
    let mem_limit: RunResponse = serde_json::from_value(mem_limit.to_owned()).unwrap();
    let out_limit: RunResponse = serde_json::from_value(out_limit.to_owned()).unwrap();
    let submit_success: RunResponse = serde_json::from_value(submit_success.to_owned()).unwrap();

    let re = re.to_parsed_response().unwrap();
    let ce = ce.to_parsed_response().unwrap();
    let rs = rs.to_parsed_response().unwrap();

    let pending = pending.to_parsed_response().unwrap();
    let started = started.to_parsed_response().unwrap();
    let mem_limit = mem_limit.to_parsed_response().unwrap();
    let out_limit = out_limit.to_parsed_response().unwrap();
    let submit_success = submit_success.to_parsed_response().unwrap();

    match (
        re,
        ce,
        rs,
        pending,
        started,
        mem_limit,
        out_limit,
        submit_success,
    ) {
        (
            ParsedResponse::RuntimeError(_),
            ParsedResponse::CompileError(_),
            ParsedResponse::Success(_),
            ParsedResponse::Pending,
            ParsedResponse::Pending,
            ParsedResponse::MemoryLimitExceeded(_),
            ParsedResponse::OutputLimitExceed(_),
            ParsedResponse::Success(_),
        ) => {
            assert!(true)
        }
        (_, _, _, _, _, _, _, _) => {
            assert!(false)
        }
    }
}
