use super::Language;
use crate::deserializers::custom_serde::status_from_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum State {
    Pending,
    Success,
    Started,
}

#[derive(Debug)]
pub enum StatusMessage {
    Accepted,
    WrongAnswer,
    MemoryLimitExceeded,
    OutputLimitExceeded,
    TimeLimitExceeded,
    RuntimeError,
    InternalError,
    CompileError,
    Timeout,
    Unknown(String),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RunResponse {
    CompileError {
        status_code: u32,
        lang: Language,
        run_success: bool,
        compile_error: String,
        full_compile_error: String,
        status_runtime: String,
        memory: u32,
        code_answer: Vec<String>,
        code_output: Vec<String>,
        std_output_list: Vec<String>,
        task_finish_time: u64,
        task_name: String,
        total_correct: Option<u32>,
        total_testcases: Option<u32>,
        runtime_percentile: Option<f32>,
        status_memory: String,
        memory_percentile: Option<f32>,
        pretty_lang: String,
        submission_id: String,

        #[serde(deserialize_with = "status_from_string")]
        status_msg: StatusMessage,
        state: String,
    },
    RuntimeError {
        status_code: u32,
        lang: Language,
        run_success: bool,
        runtime_error: String,
        full_runtime_error: String,
        status_runtime: String,
        memory: u32,
        code_answer: Vec<String>,
        code_output: Vec<String>,
        std_output_list: Vec<String>,
        elapsed_time: u32,
        task_finish_time: u64,
        task_name: String,
        total_correct: Option<u32>,
        total_testcases: Option<u32>,
        runtime_percentile: Option<f32>,
        status_memory: String,
        memory_percentile: Option<f32>,
        pretty_lang: String,
        submission_id: String,

        #[serde(deserialize_with = "status_from_string")]
        status_msg: StatusMessage,
        state: State,
    },
    Success {
        status_code: u32,
        lang: Language,
        run_success: bool,
        status_runtime: String,
        memory: u32,
        code_answer: Vec<String>,
        code_output: Vec<String>,
        std_output_list: Vec<String>,
        elapsed_time: u32,
        task_finish_time: u64,
        task_name: String,
        total_correct: Option<u32>,
        total_testcases: Option<u32>,
        runtime_percentile: Option<f32>,
        status_memory: String,
        memory_percentile: Option<f32>,
        pretty_lang: String,
        submission_id: String,

        #[serde(deserialize_with = "status_from_string")]
        status_msg: StatusMessage,
        state: State,
    },
    State {
        state: State,
    },
}
