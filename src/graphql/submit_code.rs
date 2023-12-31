use serde::{Deserialize, Serialize};

use super::{GQLLeetcodeQuery, Language};
use crate::deserializers::run_submit::RunResponse;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubmitCode {
    pub lang: Language,
    pub question_id: String,
    pub typed_code: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubmitCodeResponse {
    submission_id: u32,
}

impl GQLLeetcodeQuery for SubmitCode {
    type T = SubmitCodeResponse;

    fn get_endpoint(&self) -> String {
        let slug = self.slug.as_str();
        format!("https://leetcode.com/problems/{slug}/submit/")
    }
}

/// It may take indefinite time to run the solution on leetcode.
/// Hence polling is done to retrieve the run status from the server.
impl GQLLeetcodeQuery for SubmitCodeResponse {
    type T = RunResponse;
    fn is_post(&self) -> bool {
        false
    }

    fn get_endpoint(&self) -> String {
        let submission_id = self.submission_id;
        format!("https://leetcode.com/submissions/detail/{submission_id}/check/")
    }
}

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
        let request_body: SubmitCode = serde_json::from_str(request_json_data).unwrap();
        println!("Request Body: {:?}", request_body);

        // Define expected request body
        let expected_request_body = SubmitCode {
            lang: super::super::Language::Python3,
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
        let response_body: SubmitCodeResponse = serde_json::from_str(response_json_data).unwrap();
        println!("Response Body: {:?}", response_body);

        // Define expected response body
        let expected_response_body = SubmitCodeResponse {
            submission_id: 1001727658,
        };

        // Test if the parsed response body matches the expected response body
        assert_eq!(response_body, expected_response_body);
    }
}
