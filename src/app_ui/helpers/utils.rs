use std::path::{Path, PathBuf};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::fs::File;
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

pub(crate) struct SolutionFile<'a> {
    slug: &'a str,
    lang: &'a Language,
    description: Option<&'a str>,
    editor_data: Option<&'a str>,
    question_id: &'a str,
}

impl<'a> SolutionFile<'a> {
    pub(crate) fn new(
        slug: &'a str,
        lang: &'a Language,
        description: Option<&'a str>,
        editor_data: Option<&'a str>,
        question_id: &'a str,
    ) -> Self {
        Self {
            slug,
            lang,
            description,
            editor_data,
            question_id,
        }
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
            "s_{:0>3}_{}.{}",
            self.question_id,
            self.slug_to_snake_case(),
            self.lang.get_extension()
        )
    }

    fn slug_to_snake_case(&self) -> String {
        self.slug.replace('-', "_")
    }

    fn get_file_contents(&self) -> Option<String> {
        if let Some(description) = self.get_commented_description() {
            if let Some(editor_data) = self.editor_data {
                return Some(format!("{}\n\n{}", description, editor_data));
            }
        }
        None
    }

    fn get_commented_description(&self) -> Option<String> {
        if let Some(d) = self.description {
            return Some(self.lang.comment_text(d));
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
            slug: "two-sum",
            lang: &Language::Python3,
            description: Some("Helloworld"),
            editor_data: Some("def hello():    print('hello')"),
            question_id: "1",
        };

        assert_eq!("s_001_two_sum.py".to_string(), sf.get_file_name());
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
