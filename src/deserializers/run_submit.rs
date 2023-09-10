use crate::graphql::Language;
use crate::{deserializers::custom_serde::status_from_id, errors::AppResult};
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use strum::Display;

#[derive(Debug, Deserialize, Serialize, Display)]
#[serde(rename_all = "UPPERCASE")]
pub enum State {
    Pending,
    Success,
    Started,
}

#[derive(Debug, Display)]
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
    Unknown,
}

impl StatusMessage {
    pub fn to_status_code(&self) -> u32 {
        match self {
            StatusMessage::Accepted => 10,
            StatusMessage::WrongAnswer => 11,
            StatusMessage::MemoryLimitExceeded => 12,
            StatusMessage::OutputLimitExceeded => 13,
            StatusMessage::TimeLimitExceeded => 14,
            StatusMessage::RuntimeError => 15,
            StatusMessage::InternalError => 16,
            StatusMessage::CompileError => 20,
            StatusMessage::Timeout => 30,
            StatusMessage::Unknown => 0,
        }
    }

    pub fn from_status_code(status_code: u32) -> StatusMessage {
        match status_code {
            10 => StatusMessage::Accepted,
            11 => StatusMessage::WrongAnswer,
            12 => StatusMessage::MemoryLimitExceeded,
            13 => StatusMessage::OutputLimitExceeded,
            14 => StatusMessage::TimeLimitExceeded,
            15 => StatusMessage::RuntimeError,
            16 => StatusMessage::InternalError,
            20 => StatusMessage::CompileError,
            30 => StatusMessage::Timeout,
            _ => StatusMessage::Unknown,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RunResponse(pub serde_json::Value);

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IntermediateParsed {
    Response {
        #[serde(deserialize_with = "status_from_id")]
        status_code: Option<StatusMessage>,
    },
    Pending {
        state: State,
    },
}

impl RunResponse {
    pub fn to_parsed_response(&self) -> AppResult<ParsedResponse> {
        let value = self.0.clone();
        let value_copy = value.clone();
        let intermediate_parsed: IntermediateParsed = from_value(value)?;
        let k = match intermediate_parsed {
            IntermediateParsed::Pending { .. } => ParsedResponse::Pending,
            IntermediateParsed::Response {
                status_code: status_message,
                ..
            } => {
                let status_message = status_message.unwrap();
                let status_code = status_message.to_status_code();
                match status_message {
                    StatusMessage::Accepted | StatusMessage::WrongAnswer => {
                        ParsedResponse::Success(from_value(value_copy)?)
                    }
                    StatusMessage::MemoryLimitExceeded => {
                        ParsedResponse::MemoryLimitExceeded(from_value(value_copy)?)
                    }
                    StatusMessage::OutputLimitExceeded => {
                        ParsedResponse::OutputLimitExceed(from_value(value_copy)?)
                    }
                    StatusMessage::TimeLimitExceeded => {
                        ParsedResponse::TimeLimitExceeded(from_value(value_copy)?)
                    }
                    StatusMessage::RuntimeError => {
                        ParsedResponse::RuntimeError(from_value(value_copy)?)
                    }
                    StatusMessage::InternalError => {
                        ParsedResponse::InternalError(InternalError { status_code })
                    }
                    StatusMessage::CompileError => {
                        ParsedResponse::CompileError(from_value(value_copy)?)
                    }
                    StatusMessage::Timeout => ParsedResponse::TimeOut(Timeout { status_code }),
                    StatusMessage::Unknown => ParsedResponse::Unknown(status_code),
                }
            }
        };
        Ok(k)
    }
}

#[derive(Deserialize, Debug)]
pub enum ParsedResponse {
    Pending,
    CompileError(CompileError),
    RuntimeError(RuntimeError),
    MemoryLimitExceeded(MemoryLimitExceeded),
    OutputLimitExceed(OutputLimitExceed),
    TimeLimitExceeded(TimeLimitExceeded),
    InternalError(InternalError),
    Unknown(u32),
    TimeOut(Timeout),
    Success(Success),
}

#[derive(Deserialize, Debug)]
pub struct Timeout {
    pub status_code: u32,
}

#[derive(Deserialize, Debug)]
pub struct CompileError {
    pub lang: Language,
    pub compile_error: String,
    pub full_compile_error: String,
}

#[derive(Deserialize, Debug)]
pub struct RuntimeError {
    pub lang: Language,
    pub runtime_error: String,
    pub full_runtime_error: String,
    // pub memory: u32,
    // pub elapsed_time: u32,
}

#[derive(Deserialize, Debug)]
pub struct MemoryLimitExceeded {
    pub memory: u32,
}

#[derive(Deserialize, Debug)]
pub struct InternalError {
    pub status_code: u32,
}

#[derive(Deserialize, Debug)]
pub struct TimeLimitExceeded {
    pub elapsed_time: u32,
}

#[derive(Deserialize, Debug)]
pub struct OutputLimitExceed {
    pub memory: u32,
    pub question_id: String,
    pub compare_result: String,
    pub std_output: String,
    pub last_testcase: String,
    pub expected_output: String,
    pub finished: bool,
    pub total_correct: i32,
    pub total_testcases: i32,
    pub submission_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Success {
    Run {
        status_runtime: String,
        memory: u32,
        question_id: Option<String>,
        elapsed_time: u32,
        code_answer: Vec<String>,
        std_output_list: Vec<String>,
        expected_code_answer: Vec<String>,
        correct_answer: bool,
        total_correct: Option<u32>,
        total_testcases: Option<u32>,
        runtime_percentile: Option<f32>,
        status_memory: String,
        memory_percentile: Option<f32>,
    },
    Submit {
        status_runtime: String,
        memory: u32,
        question_id: Option<String>,
        elapsed_time: u32,
        std_output: String,
        expected_output: String,
        total_correct: Option<u32>,
        total_testcases: Option<u32>,
        runtime_percentile: Option<f32>,
        status_memory: String,
        memory_percentile: Option<f32>,
    },
}
