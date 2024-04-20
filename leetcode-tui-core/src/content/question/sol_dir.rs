use indexmap::IndexSet;
use std::{collections::HashMap, hash::Hash, path::PathBuf, sync::RwLock};
use tokio::fs::read_to_string;

use leetcode_tui_config::CONFIG;
use leetcode_core::types::language::Language;
use regex::Regex;
use std::sync::OnceLock;

use crate::errors::{CoreError, CoreResult};
pub static FILENAME_REGEX: OnceLock<regex::Regex> = OnceLock::new();
pub static SOLUTION_FILE_MANAGER: OnceLock<RwLock<SolutionFileManager>> = OnceLock::new();

pub(crate) fn init() {
    SOLUTION_FILE_MANAGER
        .get_or_init(|| RwLock::new(CONFIG.as_ref().solutions_dir.clone().try_into().unwrap()));
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SolutionFile {
    path: PathBuf,
    pub(crate) question_id: String,
    pub(crate) title_slug: String,
    pub(crate) language: Language,
}

impl Hash for SolutionFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.question_id.hash(state);
        self.language.hash(state);
    }
}

impl SolutionFile {
    pub async fn read_contents(&self) -> CoreResult<String> {
        Ok(read_to_string(&self.path).await?)
    }
}

#[derive(Debug, Default)]
pub struct SolutionFileManager {
    id_language_map: HashMap<String, IndexSet<SolutionFile>>,
}

impl SolutionFileManager {
    fn add_solution_file(&mut self, file: SolutionFile) {
        self.id_language_map
            .entry(file.question_id.clone())
            .or_default()
            .insert(file);
    }

    pub(crate) fn create_solution_file(
        &mut self,
        file_name: &str,
        contents: &str,
    ) -> CoreResult<PathBuf> {
        let sol = &CONFIG.as_ref().solutions_dir;
        let file_path = sol.as_path().join(file_name);
        if !file_path.exists() {
            std::fs::write(file_path.as_path(), contents)?;
        }
        self.add_solution_file(file_path.clone().try_into()?);
        Ok(file_path.clone())
    }

    pub(crate) fn get_available_languages(&self, question_id: &str) -> CoreResult<Vec<&Language>> {
        self.id_language_map
            .get(question_id)
            .ok_or(CoreError::QuestionIdDoesNotExist(question_id.into()))
            .map(|v| v.iter().map(|sf| &sf.language).collect::<Vec<_>>())
    }

    pub(crate) fn get_solution_file(
        &self,
        question_id: &str,
        selected: usize,
    ) -> CoreResult<&SolutionFile> {
        self.id_language_map
            .get(question_id)
            .ok_or(CoreError::QuestionIdDoesNotExist(question_id.into()))
            .map(|v| v.get_index(selected).expect("Index does not exist"))
    }
}

impl TryFrom<PathBuf> for SolutionFileManager {
    type Error = CoreError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut sfm = Self::default();
        for maybe_entry in std::fs::read_dir(value)? {
            let entry = maybe_entry?;
            let file_path = entry.path();
            let maybe_sol_file: CoreResult<SolutionFile> = file_path.try_into();
            if let Err(CoreError::FileNameDoesNotExistError(_)) = maybe_sol_file {
                continue;
            } else {
                let sol_file = maybe_sol_file?;
                sfm.add_solution_file(sol_file);
            }
        }
        Ok(sfm)
    }
}

impl TryFrom<PathBuf> for SolutionFile {
    type Error = CoreError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let regex = FILENAME_REGEX.get_or_init(|| {
            Regex::new(r"0*(?P<q_id>\d*?)_(?P<slug>[\w-]*?)_(?P<lang_id>\d+).(?P<ext>\w+)")
                .expect("Could not compile regex.")
        });

        let err = Err(CoreError::FileNameFormatDoesNotMatch(value.clone()));

        let file_name = value
            .as_path()
            .file_name()
            .ok_or(CoreError::FileNameDoesNotExistError(value.clone()))?;

        let captures = regex.captures(
            file_name
                .to_str()
                .ok_or(CoreError::Utf8ValidityError(value.clone()))?,
        );

        let ids = ["q_id", "slug", "lang_id"];
        let mut res: [Option<&str>; 3] = [None, None, None];
        if let Some(_captures) = captures {
            for (i, cap_id) in ids.iter().enumerate() {
                res[i] = _captures.name(cap_id).map(|v| v.as_str());
            }
        } else {
            return err;
        }

        if let (Some(qid), Some(slug), Some(lang_id)) = (res[0], res[1], res[2]) {
            let lang: Language = lang_id.parse::<u32>()?.into();
            Ok(Self {
                question_id: qid.to_string(),
                title_slug: slug.to_string(),
                path: value,
                language: lang,
            })
        } else {
            err
        }
    }
}
