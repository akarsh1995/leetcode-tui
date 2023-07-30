use serde::{Deserialize, Serialize};

use super::{GQLLeetcodeQuery, Language};
use crate::deserializers::run_submit::RunResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct RunCode {
    pub lang: Language,
    pub question_id: String,
    pub typed_code: String,
    #[serde(rename = "data_input")]
    pub test_cases_stdin: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RunCodeResponse {
    interpret_id: String,
    test_case: String,
}

impl GQLLeetcodeQuery for RunCode {
    type T = RunCodeResponse;

    fn get_endpoint(&self) -> String {
        let slug = self.slug.as_str();
        format!("https://leetcode.com/problems/{slug}/interpret_solution/")
    }
}

impl GQLLeetcodeQuery for RunCodeResponse {
    type T = RunResponse;
    fn is_post(&self) -> bool {
        false
    }

    fn get_endpoint(&self) -> String {
        let interpret_id = self.interpret_id.as_str();
        format!("https://leetcode.com/submissions/detail/{interpret_id}/check/")
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::RunCode;

    #[test]
    fn test() {
        let s = RunCode {
            lang: crate::graphql::Language::Python3,
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
