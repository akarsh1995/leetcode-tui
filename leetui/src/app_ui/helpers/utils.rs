use std::hash::Hash;
use std::path::{Path, PathBuf};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::fs::{read_to_string, File};
use std::io::Write;

use crate::errors::AppResult;
use crate::graphql::Language;

pub(crate) fn generate_random_string(length: usize) -> String {
    let rng = thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    random_string
}

fn write_to_file(filename: &PathBuf, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[derive(Debug)]
pub(crate) struct SolutionFile {
    slug: String,
    pub lang: Language,
    description: Option<String>,
    editor_data: Option<String>,
    pub question_id: String,
}

impl Eq for SolutionFile {}

impl PartialEq for SolutionFile {
    fn eq(&self, other: &Self) -> bool {
        self.slug == other.slug && self.lang == other.lang
    }
}

impl Hash for SolutionFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.slug.hash(state);
        self.lang.hash(state);
    }
}

pub(crate) fn slugify(input: &str) -> String {
    // Replace non-alphanumeric characters with hyphens
    let slug = input
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>();

    // Convert the slug to lowercase
    slug.to_lowercase()
}

impl SolutionFile {
    pub(crate) fn new(
        slug: String,
        lang: Language,
        description: Option<String>,
        editor_data: Option<String>,
        question_id: String,
    ) -> Self {
        Self {
            slug,
            lang,
            description,
            editor_data,
            question_id,
        }
    }

    pub fn from_file(value: PathBuf) -> Option<Self> {
        if value.is_file() {
            let file_name = value
                .file_name()
                .expect("cannot get file name")
                .to_str()
                .expect("cannot convert filename to string");
            if file_name.starts_with("s_") {
                let mut splitted = file_name.split('_');
                splitted.next();
                let id = splitted.next().unwrap();
                let slug = splitted.next().unwrap();
                let lang_id = splitted.next().unwrap().split('.').next().unwrap();
                let lang = Language::from_id(lang_id.parse().unwrap());
                return Some(Self {
                    slug: slug.to_string(),
                    lang,
                    description: None,
                    editor_data: None,
                    question_id: id.to_string(),
                });
            }
        }
        None
    }

    pub(crate) fn create_if_not_exists(&self, path: &Path) -> AppResult<()> {
        let save_path = &self.get_save_path(path);
        if !save_path.exists() {
            if let Some(contents) = self.get_file_contents() {
                write_to_file(save_path, contents.as_str())?;
            }
        }
        Ok(())
    }

    pub(crate) fn get_save_path(&self, directory: &Path) -> PathBuf {
        directory.join(self.get_file_name())
    }

    fn get_file_name(&self) -> String {
        format!(
            "s_{0:0>3}_{1}_{2}.{3}",
            self.question_id,
            self.slug,
            self.lang.to_id(),
            self.lang.get_extension()
        )
    }

    pub fn read_file_contents(&self, directory: &Path) -> String {
        let file_path = self.get_save_path(directory);
        read_to_string(file_path).unwrap()
    }

    fn get_file_contents(&self) -> Option<String> {
        if let Some(description) = self.get_commented_description() {
            if let Some(editor_data) = &self.editor_data {
                return Some(format!("{}\n\n{}", description, editor_data));
            }
        }
        None
    }

    fn get_commented_description(&self) -> Option<String> {
        if let Some(d) = &self.description {
            return Some(self.lang.comment_text(d.as_str()));
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solution() {
        let sf = SolutionFile {
            slug: "two-sum".to_string(),
            lang: Language::Python3,
            description: Some("Helloworld".to_string()),
            editor_data: Some("def hello():    print('hello')".to_string()),
            question_id: "1".to_string(),
        };

        assert_eq!("s_001_two-sum_11.py".to_string(), sf.get_file_name());
        assert_eq!(
            Some("'''\nHelloworld\n'''".to_string()),
            sf.get_commented_description()
        );
        assert_eq!(
            Some("'''\nHelloworld\n'''\n\ndef hello():    print('hello')".to_string()),
            sf.get_file_contents()
        );
    }
}
