use serde::{Deserialize, Serialize};

use super::{check_run_submit::RunResponse, GQLLeetcodeQuery, Language};

#[derive(Debug, Deserialize, Serialize)]
struct RunSolutionBody {
    lang: Language,
    question_id: String,
    typed_code: String,
    #[serde(rename = "data_input")]
    test_cases_stdin: String,
    #[serde(skip_serializing, skip_deserializing)]
    slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RunSolutionResponse {
    interpret_id: String,
    test_cases: String,
}

impl GQLLeetcodeQuery for RunSolutionBody {
    type T = RunSolutionResponse;

    fn get_endpoint(&self) -> String {
        let slug = self.slug.as_str();
        format!("https://leetcode.com/problems/{slug}/interpret_solution/")
    }
}

impl GQLLeetcodeQuery for RunSolutionResponse {
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

    use super::RunSolutionBody;

    #[test]
    fn test() {
        let s = RunSolutionBody {
            lang: crate::graphql::Language::Python3,
            question_id: "1".to_string(),
            typed_code: "class Solution:\n    def twoSum(self, nums: List[int], target: int) -> List[int]:    return [4]".to_string(),
            test_cases_stdin: "[2,7,11,15]\n9\n[3,2,4]\n6\n[3,3]\n6".to_string(),
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
