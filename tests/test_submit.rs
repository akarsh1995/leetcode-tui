use leetcode_tui_rs::graphql::check_run_submit::{RunResponse, StatusMessage};
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
    let re: RunResponse = serde_json::from_value(runtime_error.to_owned()).unwrap();
    let ce: RunResponse = serde_json::from_value(compile_error.to_owned()).unwrap();
    let rs: RunResponse = serde_json::from_value(run_success.to_owned()).unwrap();
    let pending: RunResponse = serde_json::from_value(pending.to_owned()).unwrap();
    let started: RunResponse = serde_json::from_value(started.to_owned()).unwrap();
    let mem_limit: RunResponse = serde_json::from_value(mem_limit.to_owned()).unwrap();
    let out_limit: RunResponse = serde_json::from_value(out_limit.to_owned()).unwrap();

    match out_limit {
        RunResponse::OutputLimitExceed { status_msg, .. } => match status_msg {
            StatusMessage::OutputLimitExceeded => {}
            _ => assert!(false),
        },
        _ => assert!(false),
    }

    match mem_limit {
        RunResponse::Success { status_msg, .. } => match status_msg {
            StatusMessage::MemoryLimitExceeded => {}
            _ => assert!(false),
        },
        _ => assert!(false),
    }

    match ce {
        RunResponse::CompileError { status_msg, .. } => {
            match status_msg {
                StatusMessage::CompileError => {}
                _ => assert!(false),
            }
            assert!(true)
        }
        _ => {
            dbg!(&ce);
            assert!(false);
        }
    }
    match re {
        RunResponse::RuntimeError { .. } => assert!(true),
        _ => {
            dbg!(re);
            assert!(false)
        }
    }
    match rs {
        RunResponse::Success { .. } => assert!(true),
        _ => {
            dbg!(rs);
            assert!(false);
        }
    }
    match pending {
        RunResponse::State { .. } => assert!(true),
        _ => {
            dbg!(pending);
            assert!(false)
        }
    }
    match started {
        RunResponse::State { .. } => assert!(true),
        _ => {
            dbg!(started);
            assert!(false);
        }
    }
}
