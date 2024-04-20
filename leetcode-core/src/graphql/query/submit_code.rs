use super::{GQLLeetcodeRequest, RunOrSubmitCodeCheckResult};
use crate::types::run_submit_response::RunSubmitResult;
pub use crate::types::submit::{SubmitCodeIntermediateResponse, SubmitCodeRequest};

impl GQLLeetcodeRequest for SubmitCodeRequest {
    type T = SubmitCodeIntermediateResponse;

    fn get_endpoint(&self) -> String {
        let slug = self.slug.as_str();
        format!("https://leetcode.com/problems/{slug}/submit/")
    }
}

/// Polling is done to retrieve the run status from the server. Hence it may take indefinite time to run the solution on leetcode.
impl GQLLeetcodeRequest for SubmitCodeIntermediateResponse {
    type T = RunSubmitResult;
    fn is_post(&self) -> bool {
        false
    }

    fn get_endpoint(&self) -> String {
        let submission_id = self.submission_id;
        format!("https://leetcode.com/submissions/detail/{submission_id}/check/")
    }
}

impl RunOrSubmitCodeCheckResult<SubmitCodeIntermediateResponse> for SubmitCodeRequest {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Request JSON data as a string
        let request_json_data = r#"
            {
                "lang": "python3",
                "question_id": "1",
                "typed_code": "class Solution:\n    def twoSum(self, nums: List[int], target: int) -> List[int]:    return [1]"
            }
        "#;

        // Parse Request JSON data into the RequestBody struct
        let request_body: SubmitCodeRequest = serde_json::from_str(request_json_data).unwrap();
        println!("Request Body: {:?}", request_body);

        // Define expected request body
        let expected_request_body = SubmitCodeRequest {
            lang: crate::types::language::Language::Python3,
            question_id: "1".to_string(),
            typed_code: "class Solution:\n    def twoSum(self, nums: List[int], target: int) -> List[int]:    return [1]".to_string(),
            slug: "".to_string()
        };

        // Test if the parsed request body matches the expected request body
        assert_eq!(request_body, expected_request_body);

        // Response JSON data as a string
        let response_json_data = r#"
            {
                "submission_id": 1001727658
            }
        "#;

        // Parse Response JSON data into the ResponseBody struct
        let response_body: SubmitCodeIntermediateResponse =
            serde_json::from_str(response_json_data).unwrap();
        println!("Response Body: {:?}", response_body);

        // Define expected response body
        let expected_response_body = SubmitCodeIntermediateResponse {
            submission_id: 1001727658,
        };

        // Test if the parsed response body matches the expected response body
        assert_eq!(response_body, expected_response_body);
    }
}
