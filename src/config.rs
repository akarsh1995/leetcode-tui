use std::{fs::File, io::read_to_string};

use serde::{self, Deserialize};
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub db: Db,
    pub leetcode: Leetcode,
}

impl Config {
    pub fn from_file(_path: &str) -> Self {
        let f = File::open("config.toml").unwrap();
        let contents = read_to_string(f).unwrap();
        toml::from_str(contents.as_str()).unwrap()
    }
}

#[derive(Deserialize)]
pub struct Db {
    pub url: String,
}

#[derive(Deserialize)]
pub struct Leetcode {
    #[serde(rename = "LEETCODE_SESSION")]
    pub leetcode_session: String,
    pub csrftoken: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sample_config = [
            "[db]",
            "url = 'sqlite://leetcode.sqlite'",
            "[leetcode]",
            "csrftoken = 'ctoken'",
            "LEETCODE_SESSION = 'lsession'",
        ]
        .join("\n");

        let config: Config = toml::from_str(sample_config.as_str()).unwrap();
        assert_eq!(config.leetcode.csrftoken, "ctoken".to_string());
        assert_eq!(config.leetcode.leetcode_session, "lsession".to_string());

        assert_eq!(config.db.url, "sqlite://leetcode.sqlite".to_string());
    }
}
