use std::path::PathBuf;
use tokio::fs::create_dir_all;
use tokio::io::AsyncWriteExt;
use tokio::{fs::File, io::AsyncReadExt};

use serde::{self, Deserialize, Serialize};
use toml;

#[cfg(target_family = "windows")]
use std::env;

#[cfg(target_family = "unix")]
use xdg;

use crate::errors::AppResult;

pub async fn write_file(path: PathBuf, contents: &str) -> AppResult<()> {
    let mut file = File::create(path).await?;
    file.write_all(contents.as_bytes()).await?;
    Ok(())
}

#[cfg(target_family = "windows")]
fn get_home_directory() -> String {
    env::var("USERPROFILE")
        .ok()
        .expect("Cannot find the env var USERPROFILE")
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub db: Db,
    pub leetcode: Leetcode,
    pub solutions_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let solutions_dir = Self::get_default_solutions_dir().expect("Cannot config base dir");
        Self {
            db: Default::default(),
            leetcode: Default::default(),
            solutions_dir,
        }
    }
}

impl Config {
    #[cfg(target_family = "windows")]
    pub fn get_config_base_directory() -> AppResult<PathBuf> {
        let mut home = PathBuf::new();
        home.push(get_home_directory());
        home.push(Self::get_base_name());
        Ok(home)
    }

    #[cfg(target_family = "windows")]
    pub fn get_data_base_directory() -> AppResult<PathBuf> {
        Self::get_config_base_directory()
    }

    #[cfg(target_family = "unix")]
    pub fn get_config_base_directory() -> AppResult<PathBuf> {
        Ok(xdg::BaseDirectories::with_prefix(Self::get_base_name())?.get_config_home())
    }

    #[cfg(target_family = "unix")]
    pub fn get_data_base_directory() -> AppResult<PathBuf> {
        Ok(xdg::BaseDirectories::with_prefix(Self::get_base_name())?.get_data_home())
    }

    pub fn get_base_name() -> &'static str {
        "leetcode_tui"
    }

    pub fn get_default_solutions_dir() -> AppResult<PathBuf> {
        let mut path = Self::get_config_base_directory()?;
        path.push("solutions");
        Ok(path)
    }

    pub async fn create_solutions_dir() -> AppResult<()> {
        let default = Self::get_default_solutions_dir()?;
        Ok(create_dir_all(default).await?)
    }

    pub fn get_config_base_file() -> AppResult<PathBuf> {
        let mut base_config_dir = Self::get_config_base_directory()?;
        base_config_dir.push("config.toml");
        Ok(base_config_dir)
    }

    pub async fn read_config(path: PathBuf) -> AppResult<Self> {
        let mut f = File::open(path).await?;
        let mut contents = String::new();
        f.read_to_string(&mut contents).await?;
        Ok(toml::from_str(contents.as_str())?)
    }

    pub async fn write_config(&self, path: PathBuf) -> AppResult<()> {
        create_dir_all(
            path.parent()
                .unwrap_or_else(|| panic!("Cannot get parent dir of: {}", path.display())),
        )
        .await?;
        write_file(path, toml::to_string(self)?.as_str()).await?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Db {
    pub url: String,
}

impl Db {
    pub fn get_base_sqlite_data_path() -> AppResult<PathBuf> {
        let mut db_path = Config::get_data_base_directory()?;
        db_path.push("data.sqlite");
        Ok(db_path)
    }

    pub async fn touch_default_db() -> AppResult<()> {
        let path = Self::get_base_sqlite_data_path()?;
        create_dir_all(
            path.clone()
                .parent()
                .expect("cannot get the parent directory"),
        )
        .await?;
        write_file(path, "").await?;
        Ok(())
    }
}

impl Default for Db {
    fn default() -> Self {
        Self {
            url: format!(
                "sqlite://{}",
                Self::get_base_sqlite_data_path()
                    .expect("cannot place sqlite data file")
                    .display()
            ),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Leetcode {
    #[serde(rename = "LEETCODE_SESSION")]
    pub leetcode_session: String,
    pub csrftoken: String,
}

impl Default for Leetcode {
    fn default() -> Self {
        Self {
            leetcode_session: "".to_owned(),
            csrftoken: "".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sample_config = [
            "solutions_dir = '/some/xyz/path'",
            "[db]",
            "url = 'sqlite://leetcode.sqlite'",
            "[leetcode]",
            "csrftoken = 'ctoken'",
            "LEETCODE_SESSION = 'lsession'",
        ]
        .join("\n");

        let mut pathbuf = PathBuf::new();
        pathbuf.push("/");
        pathbuf.push("some");
        pathbuf.push("xyz");
        pathbuf.push("path");
        let config: Config = toml::from_str(sample_config.as_str()).unwrap();
        assert_eq!(config.solutions_dir, pathbuf);
        assert_eq!(config.leetcode.csrftoken, "ctoken".to_string());
        assert_eq!(config.leetcode.leetcode_session, "lsession".to_string());

        assert_eq!(config.db.url, "sqlite://leetcode.sqlite".to_string());
    }
}
