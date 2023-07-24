use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
pub mod check_run_submit;
pub mod problemset_question_list;
pub mod question_content;
pub mod run_code;
pub mod submit;
use crate::errors::AppResult;

pub type QuestionContentQuery = question_content::Query;

#[async_trait]
pub trait GQLLeetcodeQuery: Serialize + Sync {
    type T: DeserializeOwned;

    fn get_body(&self) -> Value {
        json!(self)
    }

    fn is_post(&self) -> bool {
        true
    }

    /// Default graphql endpoint
    fn get_endpoint(&self) -> String {
        "https://leetcode.com/graphql".to_string()
    }

    async fn post(&self, client: &reqwest::Client) -> AppResult<Self::T> {
        let request;
        if self.is_post() {
            request = client.post(self.get_endpoint()).json(&self.get_body());
        } else {
            request = client.get(self.get_endpoint());
        }
        Ok(request
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?)
    }
}

pub enum RunOrSubmitCode {
    Run(RunSolutionBody),
    Submit(SubmitRequestBody),
}

impl RunOrSubmitCode {
    pub async fn post(&self, client: &reqwest::Client) -> AppResult<RunResponse> {
        match self {
            RunOrSubmitCode::Run(run) => self.poll_check_response(client, run).await,
            RunOrSubmitCode::Submit(submit) => self.poll_check_response(client, submit).await,
        }
    }

    pub async fn poll_check_response<T: GQLLeetcodeQuery<T = RunResponse>>(
        &self,
        client: &reqwest::Client,
        body: &impl GQLLeetcodeQuery<T = T>,
    ) -> AppResult<RunResponse> {
        let run_response: T = body.post(client).await?;
        loop {
            let status_check = run_response.post(client).await?;
            match status_check {
                RunResponse::State { .. } => {}
                _ => return Ok(status_check),
            }
        }
    }
}

use serde::Deserialize;

use self::{check_run_submit::RunResponse, run_code::RunSolutionBody, submit::SubmitRequestBody};

#[derive(Debug, Deserialize, Serialize)]
struct LanguageInfo {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    language_list: Vec<LanguageInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Languages {
    data: Data,
}

// Generate the enum for languages
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Cpp,
    Java,
    Python,
    Python3,
    Mysql,
    Mssql,
    Oraclesql,
    C,
    Csharp,
    Javascript,
    Ruby,
    Bash,
    Swift,
    Golang,
    Scala,
    Html,
    Pythonml,
    Kotlin,
    Rust,
    Php,
    Typescript,
    Racket,
    Erlang,
    Elixir,
    Dart,
    Pythondata,
    React,
    Unknown(u32),
}

impl Language {
    fn from_id(id: u32) -> Language {
        match id {
            0 => Language::Cpp,
            1 => Language::Java,
            2 => Language::Python,
            11 => Language::Python3,
            3 => Language::Mysql,
            14 => Language::Mssql,
            15 => Language::Oraclesql,
            4 => Language::C,
            5 => Language::Csharp,
            6 => Language::Javascript,
            7 => Language::Ruby,
            8 => Language::Bash,
            9 => Language::Swift,
            10 => Language::Golang,
            12 => Language::Scala,
            16 => Language::Html,
            17 => Language::Pythonml,
            13 => Language::Kotlin,
            18 => Language::Rust,
            19 => Language::Php,
            20 => Language::Typescript,
            21 => Language::Racket,
            22 => Language::Erlang,
            23 => Language::Elixir,
            24 => Language::Dart,
            25 => Language::Pythondata,
            26 => Language::React,
            _ => Language::Unknown(id),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Language::Cpp => "cpp".to_string(),
            Language::Java => "java".to_string(),
            Language::Python => "python".to_string(),
            Language::Python3 => "python3".to_string(),
            Language::Mysql => "mysql".to_string(),
            Language::Mssql => "mssql".to_string(),
            Language::Oraclesql => "oraclesql".to_string(),
            Language::C => "c".to_string(),
            Language::Csharp => "csharp".to_string(),
            Language::Javascript => "javascript".to_string(),
            Language::Ruby => "ruby".to_string(),
            Language::Bash => "bash".to_string(),
            Language::Swift => "swift".to_string(),
            Language::Golang => "golang".to_string(),
            Language::Scala => "scala".to_string(),
            Language::Html => "html".to_string(),
            Language::Pythonml => "pythonml".to_string(),
            Language::Kotlin => "kotlin".to_string(),
            Language::Rust => "rust".to_string(),
            Language::Php => "php".to_string(),
            Language::Typescript => "typescript".to_string(),
            Language::Racket => "racket".to_string(),
            Language::Erlang => "erlang".to_string(),
            Language::Elixir => "elixir".to_string(),
            Language::Dart => "dart".to_string(),
            Language::Pythondata => "pythondata".to_string(),
            Language::React => "react".to_string(),
            Language::Unknown(id) => format!("Unknown({})", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    #[test]
    fn test() {
        // JSON data as a string
        let json_data = r#"
        {
            "data": {
                "languageList": [
                    { "id": 0, "name": "cpp" },
                    { "id": 1, "name": "java" },
                    { "id": 2, "name": "python" },
                    { "id": 11, "name": "python3" },
                    { "id": 3, "name": "mysql" },
                    { "id": 14, "name": "mssql" },
                    { "id": 15, "name": "oraclesql" },
                    { "id": 4, "name": "c" },
                    { "id": 5, "name": "csharp" },
                    { "id": 6, "name": "javascript" },
                    { "id": 7, "name": "ruby" },
                    { "id": 8, "name": "bash" },
                    { "id": 9, "name": "swift" },
                    { "id": 10, "name": "golang" },
                    { "id": 12, "name": "scala" },
                    { "id": 16, "name": "html" },
                    { "id": 17, "name": "pythonml" },
                    { "id": 13, "name": "kotlin" },
                    { "id": 18, "name": "rust" },
                    { "id": 19, "name": "php" },
                    { "id": 20, "name": "typescript" },
                    { "id": 21, "name": "racket" },
                    { "id": 22, "name": "erlang" },
                    { "id": 23, "name": "elixir" },
                    { "id": 24, "name": "dart" },
                    { "id": 25, "name": "pythondata" },
                    { "id": 26, "name": "react" }
                ]
            }
        }
    "#;

        // Parse JSON data into the Languages struct
        let languages: Languages = serde_json::from_str(json_data).unwrap();

        // Extract the languageList
        let language_list = languages.data.language_list;

        // Create a HashMap to store Language enums by id
        let mut language_map: HashMap<u32, Language> = HashMap::new();
        for lang_info in language_list {
            language_map.insert(lang_info.id, Language::from_id(lang_info.id));
        }

        // Example: Accessing the language by id
        let id_to_find = 2; // Example: "python"
        if let Some(lang) = language_map.get(&id_to_find) {
            println!("Language with id {}: {:?}", id_to_find, lang);
        }
    }
}
