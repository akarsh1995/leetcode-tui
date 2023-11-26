use super::*;

trait CustomDisplay {
    fn display(&self) -> String;
}

trait CodeSubmissionOutput {
    fn get_heading(&self) -> String;

    fn get_memory_usage_string(&self) -> String {
        format!("Memory used: {}", self.get_memory_usage())
    }

    fn run_time_taken_string(&self) -> String {
        format!("Solution runtime: {}", self.run_time_taken())
    }

    fn get_memory_usage(&self) -> &Memory;

    fn run_time_taken(&self) -> &str;

    fn display(&self) -> String {
        [
            self.get_heading(),
            self.get_memory_usage_string(),
            self.run_time_taken_string(),
            self.extra_lines().join("\n"),
        ]
        .join("\n")
        .trim()
        .to_string()
    }

    fn extra_lines(&self) -> Vec<String> {
        vec![]
    }
}

impl CodeSubmissionOutput for RunAccepted {
    fn get_heading(&self) -> String {
        format!(
            "Solution ran successfully for {}/{} cases.",
            self.total_correct, self.total_testcases
        )
    }

    fn get_memory_usage(&self) -> &Memory {
        &self.memory
    }

    fn run_time_taken(&self) -> &str {
        self.status_runtime.as_str()
    }
}

impl CodeSubmissionOutput for RunWrongAnswer {
    fn get_heading(&self) -> String {
        format!(
            "Test Run Failed: {}/{} cases passed.",
            self.total_correct, self.total_testcases
        )
    }

    fn get_memory_usage(&self) -> &Memory {
        &self.memory
    }

    fn run_time_taken(&self) -> &str {
        self.status_runtime.as_str()
    }
}

impl SubmitAccepted {
    fn run_time_percentile(&self) -> String {
        format!(
            "Your runtime beats {} % of the submissions.",
            self.runtime_percentile
        )
    }

    fn memory_percentile(&self) -> String {
        format!(
            "Your memory usage beats {} % of the submissions.",
            self.memory_percentile
        )
    }
}

impl CodeSubmissionOutput for SubmitAccepted {
    fn get_heading(&self) -> String {
        format!(
            "Solution ran successfully for {}/{} cases.",
            self.total_correct, self.total_testcases
        )
    }

    fn get_memory_usage(&self) -> &Memory {
        &self.memory
    }

    fn run_time_taken(&self) -> &str {
        self.status_runtime.as_str()
    }

    fn extra_lines(&self) -> Vec<String> {
        vec![self.run_time_percentile(), self.memory_percentile()]
    }
}

impl CodeSubmissionOutput for SubmitWrongAnswer {
    fn get_heading(&self) -> String {
        format!(
            "{}/{} cases passed.",
            self.total_correct, self.total_testcases
        )
    }

    fn get_memory_usage(&self) -> &Memory {
        &self.memory
    }

    fn run_time_taken(&self) -> &str {
        self.status_runtime.as_str()
    }
}

impl CustomDisplay for Timeout {
    fn display(&self) -> String {
        "Timed out".to_string()
    }
}

impl CustomDisplay for CompileError {
    fn display(&self) -> String {
        format!("Compile Error: {}", self.full_compile_error)
    }
}

impl CustomDisplay for RuntimeError {
    fn display(&self) -> String {
        format!("Runtime Error: {}", self.full_runtime_error)
    }
}

impl CustomDisplay for MemoryLimitExceeded {
    fn display(&self) -> String {
        format!("Memory Limit Exceeded: {}", self.memory)
    }
}

impl CustomDisplay for InternalError {
    fn display(&self) -> String {
        format!("Internal Error: Status Code: {}", self.status_code)
    }
}

impl CustomDisplay for TimeLimitExceeded {
    fn display(&self) -> String {
        format!("Time Limit Exceeded: Time Elapsed: {}", self.elapsed_time)
    }
}

impl CustomDisplay for OutputLimitExceed {
    fn display(&self) -> String {
        format!(
            "Output Limit Exceeded: Last Testcase: {:?}\nExpected Output:{:?}\nStd Output: {:?}\nCode Output: {:?}",
            self.last_testcase, self.expected_output, self.std_output, self.code_output
        )
    }
}

macro_rules! display_impl {
    (($($ResponseType: ident),*)) => {
        $(
            impl std::fmt::Display for $ResponseType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.display())
                }
            }
        )*
    };
}

display_impl!((
    RunAccepted,
    RunWrongAnswer,
    SubmitAccepted,
    SubmitWrongAnswer,
    Timeout,
    CompileError,
    RuntimeError,
    MemoryLimitExceeded,
    InternalError,
    TimeLimitExceeded,
    OutputLimitExceed
));

impl std::fmt::Display for ParsedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: String = match self {
            ParsedResponse::RunAccepted(ra) => ra.to_string(),
            ParsedResponse::SubmitAccepted(sa) => sa.to_string(),
            ParsedResponse::RunWrongAnswer(rwa) => rwa.to_string(),
            ParsedResponse::SubmitWrongAnswer(swa) => swa.to_string(),
            ParsedResponse::Pending => self.to_string(),
            ParsedResponse::CompileError(ce) => ce.to_string(),
            ParsedResponse::RuntimeError(re) => re.to_string(),
            ParsedResponse::MemoryLimitExceeded(mle) => mle.to_string(),
            ParsedResponse::OutputLimitExceed(ole) => ole.to_string(),
            ParsedResponse::TimeLimitExceeded(tle) => tle.to_string(),
            ParsedResponse::InternalError(ie) => ie.to_string(),
            ParsedResponse::Unknown(e) => format!("Unknown Error with Status Code: {e}"),
            ParsedResponse::TimeOut(to) => to.to_string(),
        };
        write!(f, "{res}")
    }
}
