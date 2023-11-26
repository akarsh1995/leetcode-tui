use super::language::Language;
use crate::errors::AppResult;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::from_value;
use strum::Display;
pub mod display;

#[derive(Debug, Deserialize, Serialize, Display)]
#[serde(rename_all = "UPPERCASE")]
pub enum State {
    Pending,
    Success,
    Started,
}

#[derive(Debug, Display, Clone)]
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

impl From<StatusMessage> for u32 {
    fn from(value: StatusMessage) -> Self {
        match value {
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
}

impl From<u32> for StatusMessage {
    fn from(value: u32) -> Self {
        match value {
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

pub(crate) fn status_from_id<'de, D>(deserializer: D) -> Result<StatusMessage, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(u32::deserialize(deserializer)?.into())
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IntermediateParsed {
    Response {
        #[serde(deserialize_with = "status_from_id")]
        status_code: StatusMessage,
    },
    Pending {
        state: State,
    },
}

#[derive(Debug, Deserialize)]
pub struct RunSubmitResult(pub serde_json::Value);

impl RunSubmitResult {
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
                let status_code = status_message.clone().into();
                match status_message {
                    StatusMessage::Accepted => {
                        let potentially_run_or_submit = value_copy.clone();
                        let compile_result: CompileResult = from_value(value_copy)?;
                        if compile_result.is_run() {
                            if compile_result.is_run_success() {
                                ParsedResponse::RunAccepted(from_value(potentially_run_or_submit)?)
                            } else {
                                ParsedResponse::RunWrongAnswer(from_value(
                                    potentially_run_or_submit,
                                )?)
                            }
                        } else {
                            ParsedResponse::SubmitAccepted(from_value(potentially_run_or_submit)?)
                        }
                    }
                    StatusMessage::WrongAnswer => {
                        ParsedResponse::SubmitWrongAnswer(from_value(value_copy)?)
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
    RunAccepted(RunAccepted),
    SubmitAccepted(SubmitAccepted),
    RunWrongAnswer(RunWrongAnswer),
    SubmitWrongAnswer(SubmitWrongAnswer),
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
}

#[derive(Deserialize, Debug)]
pub struct MemoryLimitExceeded {
    pub memory: Memory,
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
    pub memory: Memory,
    pub question_id: String,
    pub compare_result: String,
    pub std_output: String,
    pub last_testcase: String,
    pub expected_output: String,
    pub finished: bool,
    pub code_output: String,
    pub total_correct: i32,
    pub total_testcases: i32,
    pub submission_id: String,
}

#[derive(Deserialize, Debug)]
pub struct CompileResult {
    task_name: String,
    compare_result: String,
}

impl CompileResult {
    fn is_run(&self) -> bool {
        self.task_name.contains("RunCode")
    }

    fn is_correct(&self) -> bool {
        self.compare_result.chars().filter(|c| *c == '1').count() == self.compare_result.len()
    }

    fn is_run_success(&self) -> bool {
        self.is_run() && self.is_correct()
    }
}

#[derive(Deserialize, Debug)]
pub struct Memory(u32);

impl Memory {
    fn to_megabytes(&self) -> f32 {
        self.0 as f32 / 1000000.0
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} MB", self.to_megabytes())
    }
}

#[derive(Deserialize, Debug)]
pub struct RunAccepted {
    pub status_runtime: String,
    pub memory: Memory,
    pub elapsed_time: u32,
    pub code_answer: Vec<String>,
    pub std_output_list: Vec<String>,
    pub expected_code_answer: Vec<String>,
    pub correct_answer: bool,
    pub total_correct: u32,
    pub total_testcases: u32,
    pub runtime_percentile: Option<f32>,
    pub memory_percentile: Option<f32>,
    pub status_memory: String,
    pub status_msg: String,
}

#[derive(Deserialize, Debug)]
pub struct RunWrongAnswer {
    pub status_runtime: String,
    pub memory: Memory,
    pub elapsed_time: u32,
    pub code_answer: Vec<String>,
    pub std_output_list: Vec<String>,
    pub expected_code_answer: Vec<String>,
    pub correct_answer: bool,
    pub total_correct: u32,
    pub total_testcases: u32,
    pub runtime_percentile: Option<f32>,
    pub memory_percentile: Option<f32>,
    pub status_memory: String,
    pub status_msg: String,
}

#[derive(Deserialize, Debug)]
pub struct SubmitAccepted {
    pub status_runtime: String,
    pub memory: Memory,
    pub question_id: String,
    pub elapsed_time: u32,
    pub std_output: String,
    pub expected_output: String,
    pub total_correct: u32,
    pub total_testcases: u32,
    pub runtime_percentile: f32,
    pub status_memory: String,
    pub memory_percentile: f32,
    pub status_msg: String,
}

#[derive(Deserialize, Debug)]
pub struct SubmitWrongAnswer {
    pub status_runtime: String,
    pub memory: Memory,
    pub question_id: String,
    pub elapsed_time: u32,
    pub std_output: String,
    pub total_correct: u32,
    pub total_testcases: u32,
    pub status_memory: String,
    pub status_msg: String,
    pub last_testcase: String,
    pub expected_output: String,
}
