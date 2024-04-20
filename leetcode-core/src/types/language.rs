use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
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

impl From<u32> for Language {
    fn from(value: u32) -> Self {
        match value {
            0 => Language::Cpp,
            1 => Language::Java,
            2 => Language::Python,
            3 => Language::Mysql,
            4 => Language::C,
            5 => Language::Csharp,
            6 => Language::Javascript,
            7 => Language::Ruby,
            8 => Language::Bash,
            9 => Language::Swift,
            10 => Language::Golang,
            11 => Language::Python3,
            12 => Language::Scala,
            13 => Language::Kotlin,
            14 => Language::Mssql,
            15 => Language::Oraclesql,
            16 => Language::Html,
            17 => Language::Pythonml,
            18 => Language::Rust,
            19 => Language::Php,
            20 => Language::Typescript,
            21 => Language::Racket,
            22 => Language::Erlang,
            23 => Language::Elixir,
            24 => Language::Dart,
            25 => Language::Pythondata,
            26 => Language::React,
            _ => Language::Unknown(value),
        }
    }
}

impl From<Language> for u32 {
    fn from(value: Language) -> Self {
        match value {
            Language::Cpp => 0,
            Language::Java => 1,
            Language::Python => 2,
            Language::Mysql => 3,
            Language::C => 4,
            Language::Csharp => 5,
            Language::Javascript => 6,
            Language::Ruby => 7,
            Language::Bash => 8,
            Language::Swift => 9,
            Language::Golang => 10,
            Language::Python3 => 11,
            Language::Scala => 12,
            Language::Kotlin => 13,
            Language::Mssql => 14,
            Language::Oraclesql => 15,
            Language::Html => 16,
            Language::Pythonml => 17,
            Language::Rust => 18,
            Language::Php => 19,
            Language::Typescript => 20,
            Language::Racket => 21,
            Language::Erlang => 22,
            Language::Elixir => 23,
            Language::Dart => 24,
            Language::Pythondata => 25,
            Language::React => 26,
            Language::Unknown(id) => id,
        }
    }
}

impl Language {
    pub fn from_id(id: u32) -> Language {
        match id {
            0 => Language::Cpp,
            1 => Language::Java,
            2 => Language::Python,
            3 => Language::Mysql,
            4 => Language::C,
            5 => Language::Csharp,
            6 => Language::Javascript,
            7 => Language::Ruby,
            8 => Language::Bash,
            9 => Language::Swift,
            10 => Language::Golang,
            11 => Language::Python3,
            12 => Language::Scala,
            13 => Language::Kotlin,
            14 => Language::Mssql,
            15 => Language::Oraclesql,
            16 => Language::Html,
            17 => Language::Pythonml,
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

    pub fn to_id(&self) -> u32 {
        match self {
            Language::Cpp => 0,
            Language::Java => 1,
            Language::Python => 2,
            Language::Mysql => 3,
            Language::C => 4,
            Language::Csharp => 5,
            Language::Javascript => 6,
            Language::Ruby => 7,
            Language::Bash => 8,
            Language::Swift => 9,
            Language::Golang => 10,
            Language::Python3 => 11,
            Language::Scala => 12,
            Language::Kotlin => 13,
            Language::Mssql => 14,
            Language::Oraclesql => 15,
            Language::Html => 16,
            Language::Pythonml => 17,
            Language::Rust => 18,
            Language::Php => 19,
            Language::Typescript => 20,
            Language::Racket => 21,
            Language::Erlang => 22,
            Language::Elixir => 23,
            Language::Dart => 24,
            Language::Pythondata => 25,
            Language::React => 26,
            Language::Unknown(id) => *id,
        }
    }

    pub fn comment_text(&self, input_text: &str) -> String {
        let (comment_start, comment_end) = match self {
            Language::Cpp
            | Language::C
            | Language::Scala
            | Language::Java
            | Language::Javascript
            | Language::Swift
            | Language::Golang
            | Language::Rust
            | Language::Kotlin => ("/*\n", "\n*/"),
            Language::Python | Language::Python3 => ("'''\n", "\n'''"),
            Language::Mysql | Language::Mssql | Language::Oraclesql => ("-- ", ""),
            Language::Csharp => ("// ", ""),
            Language::Ruby => ("=begin\n", "\n=end"),
            Language::Bash => ("# ", ""),
            Language::Html => ("<!--\n", "\n-->"),
            Language::Pythonml => ("# ", ""),
            // => ("// ", ""),
            Language::Php => ("// ", ""),
            Language::Typescript => ("// ", ""),
            Language::Racket => ("; ", ""),
            Language::Erlang => ("% ", ""),
            Language::Elixir => ("# ", ""),
            Language::Dart => ("// ", ""),
            Language::Pythondata => ("# ", ""),
            Language::React => ("// ", ""),
            Language::Unknown(_) => ("", ""),
        };

        match self {
            Language::C
            | Language::Html
            | Language::Cpp
            | Language::Python
            | Language::Python3
            | Language::Ruby
            | Language::Javascript
            | Language::Scala
            | Language::Java
            | Language::Swift
            | Language::Golang
            | Language::Kotlin
            | Language::Rust => {
                format!("{}{}{}", comment_start, input_text, comment_end)
            }
            _ => {
                let commented_lines: Vec<String> = input_text
                    .lines()
                    .map(|line| format!("{}{}", comment_start, line))
                    .collect();

                commented_lines.join("\n")
            }
        }
    }

    pub fn get_extension(&self) -> &str {
        match self {
            Language::Cpp => "cpp",
            Language::Java => "java",
            Language::Python => "py",
            Language::Python3 => "py",
            Language::Mysql => "sql",
            Language::Mssql => "sql",
            Language::Oraclesql => "sql",
            Language::C => "c",
            Language::Csharp => "cs",
            Language::Javascript => "js",
            Language::Ruby => "rb",
            Language::Bash => "sh",
            Language::Swift => "swift",
            Language::Golang => "go",
            Language::Scala => "scala",
            Language::Html => "html",
            Language::Pythonml => "py",
            Language::Kotlin => "kt",
            Language::Rust => "rs",
            Language::Php => "php",
            Language::Typescript => "ts",
            Language::Racket => "rkt",
            Language::Erlang => "erl",
            Language::Elixir => "ex",
            Language::Dart => "dart",
            Language::Pythondata => "py",
            Language::React => "jsx",
            Language::Unknown(_) => "",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let k = match self {
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
            Language::Unknown(id) => format!("{}", id),
        };
        f.write_str(k.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_text() {
        let test_cases = [
            // Test for C++
            (
                Language::Cpp,
                "This is a single-line comment.",
                "/*\nThis is a single-line comment.\n*/",
            ),
            (
                Language::Cpp,
                "This is a multi-line text.\nIt can have multiple lines.",
                "/*\nThis is a multi-line text.\nIt can have multiple lines.\n*/",
            ),
            (
                Language::Python,
                "This is a single-line comment.",
                "'''\nThis is a single-line comment.\n'''",
            ),
            (
                Language::Python,
                "This is a multi-line text.\nIt can have multiple lines.",
                "'''\nThis is a multi-line text.\nIt can have multiple lines.\n'''",
            ),
            // Test for C
            (
                Language::C,
                "This is a single-line comment.",
                "/*\nThis is a single-line comment.\n*/",
            ),
            (
                Language::C,
                "This is a multi-line text.\nIt can have multiple lines.",
                "/*\nThis is a multi-line text.\nIt can have multiple lines.\n*/",
            ),
            // Test for HTML
            (
                Language::Html,
                "This is a single-line comment.",
                "<!--\nThis is a single-line comment.\n-->",
            ),
            (
                Language::Html,
                "This is a multi-line text.\nIt can have multiple lines.",
                "<!--\nThis is a multi-line text.\nIt can have multiple lines.\n-->",
            ),
            // Test for Unknown language
            (
                Language::Unknown(999),
                "This is a single-line comment.",
                "This is a single-line comment.",
            ),
            (
                Language::Unknown(999),
                "This is a multi-line text.\nIt can have multiple lines.",
                "This is a multi-line text.\nIt can have multiple lines.",
            ),
        ];

        for (language, input_text, expected_output) in &test_cases {
            assert_eq!(language.comment_text(input_text), *expected_output);
        }
    }

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
