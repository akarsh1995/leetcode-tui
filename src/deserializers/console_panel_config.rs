use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question_frontend_id: String,
    pub question_title: String,
    pub example_testcase_list: Vec<String>,
    // pub meta_data: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Data {
    pub question: Question,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Root {
    pub data: Data,
}

#[test]
fn test_parse_json() {
    fn parse_json(json_str: &str) -> Root {
        serde_json::from_str(json_str).unwrap()
    }
    let json_str = r#"
    {
        "data": {
            "question": {
                "questionFrontendId": "1",
                "questionTitle": "Two Sum",
                "exampleTestcaseList": [
                    "[2,7,11,15]\n9",
                    "[3,2,4]\n6",
                    "[3,3]\n6"
                ]
            }
        }
    }
"#;

    let expected_question = Question {
        question_frontend_id: "1".to_string(),
        question_title: "Two Sum".to_string(),
        example_testcase_list: vec![
            "[2,7,11,15]\n9".to_string(),
            "[3,2,4]\n6".to_string(),
            "[3,3]\n6".to_string(),
        ],
        // meta_data: "{\n  \"name\": \"twoSum\",\n  \"params\": [\n    {\n      \"name\": \"nums\",\n      \"type\": \"integer[]\"\n    },\n    {\n      \"name\": \"target\",\n      \"type\": \"integer\"\n    }\n  ],\n  \"return\": {\n    \"type\": \"integer[]\",\n    \"size\": 2\n  },\n  \"manual\": false\n}"
        //     .to_string(),
    };

    let expected_data = Data {
        question: expected_question,
    };

    let expected_root = Root {
        data: expected_data,
    };

    let root = parse_json(json_str);
    assert_eq!(root, expected_root);
}
