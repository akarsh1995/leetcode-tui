use super::{GQLLeetcodeRequest, RunOrSubmitCodeCheckResult};
pub use crate::types::{
    run::{RunCodeIntermediateResponse, RunCodeRequest},
    run_submit_response::RunSubmitResult,
};
use crate::{errors::LcAppError, graphql::query::console_panel_config};

impl GQLLeetcodeRequest for RunCodeRequest {
    type T = RunCodeIntermediateResponse;

    fn get_endpoint(&self) -> String {
        let slug = self.slug.as_str();
        format!("https://leetcode.com/problems/{slug}/interpret_solution/")
    }
}

impl GQLLeetcodeRequest for RunCodeIntermediateResponse {
    type T = RunSubmitResult;
    fn is_post(&self) -> bool {
        false
    }

    fn get_endpoint(&self) -> String {
        let interpret_id = self.interpret_id.as_str();
        format!("https://leetcode.com/submissions/detail/{interpret_id}/check/")
    }
}

impl RunCodeRequest {
    pub async fn set_sample_test_cases_if_none(&mut self) -> Result<(), LcAppError> {
        if self.test_cases_stdin.is_none() {
            let fetched_test_cases = console_panel_config::Query::new(self.slug.clone())
                .send()
                .await?
                .data
                .question
                .example_testcase_list
                .join("\n");
            self.test_cases_stdin = Some(fetched_test_cases);
        }
        Ok(())
    }
}

impl RunOrSubmitCodeCheckResult<RunCodeIntermediateResponse> for RunCodeRequest {}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::RunCodeRequest;

    #[test]
    fn test() {
        let s = RunCodeRequest {
            lang: crate::types::language::Language::Python3,
            question_id: "1".to_string(),
            typed_code: "class Solution:\n    def twoSum(self, nums: List[int], target: int) -> List[int]:    return [4]".to_string(),
            test_cases_stdin: Some("[2,7,11,15]\n9\n[3,2,4]\n6\n[3,3]\n6".to_string()),
            slug: "".to_string(),
        };

        let from_struct = json!(s);
        let from_raw_json = json!(
            {
             "lang": "python3",
             "question_id": "1",
             "typed_code": "class Solution:\n    def twoSum(self, nums: List[int], target: int) -> List[int]:    return [4]",
             "data_input": "[2,7,11,15]\n9\n[3,2,4]\n6\n[3,3]\n6"
            }
        );
        assert_eq!(from_struct, from_raw_json);
    }
}
