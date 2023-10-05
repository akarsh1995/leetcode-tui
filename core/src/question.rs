use crate::SendError;
use crate::{emit, utils::Paginate};
use config::log;
use config::CONFIG;
use config::DB_CLIENT;
use config::REQ_CLIENT;
use leetcode_core::graphql::query::RunOrSubmitCodeCheckResult;
use leetcode_core::types::language::Language;
use leetcode_core::{GQLLeetcodeRequest, QuestionContentRequest, RunCodeRequest};
use leetcode_db::{DbQuestion, DbTopic};
pub(super) mod sol_dir;
pub(crate) use sol_dir::init;

use self::sol_dir::{SolutionFileManager, SOLUTION_FILE_MANAGER};

pub struct Questions {
    paginate: Paginate<DbQuestion>,
}

impl Default for Questions {
    fn default() -> Self {
        // let solutions_manager = CONFIG.as_ref().solutions_dir.clone().try_into().unwrap();
        Self {
            // solutions_manager,
            paginate: Paginate::new(vec![]),
        }
    }
}

impl Questions {
    pub fn prev_ques(&mut self) -> bool {
        self.paginate.prev_elem()
    }

    pub fn next_ques(&mut self) -> bool {
        self.paginate.next_elem()
    }

    pub fn window(&self) -> &[DbQuestion] {
        self.paginate.window()
    }

    pub fn hovered(&self) -> Option<&DbQuestion> {
        self.paginate.hovered()
    }
}

impl Questions {
    pub fn get_questions_by_topic(&mut self, topic: DbTopic) {
        tokio::spawn(async move {
            let questions = topic.fetch_questions(DB_CLIENT.as_ref()).await;
            match questions {
                Ok(_questions) => {
                    emit!(Questions(_questions));
                }
                Err(e) => {
                    log::error!("Problem fetching questions for topic {topic:?}: {e}");
                    emit!(Error(e.to_string()));
                }
            }
        });
    }

    pub fn show_question_content(&self) -> bool {
        if let Some(_hovered) = self.hovered() {
            let slug = _hovered.title_slug.clone();
            tokio::spawn(async move {
                let qc = QuestionContentRequest::new(slug);
                let content = qc.send(REQ_CLIENT.as_ref()).await;
                match content {
                    Ok(c) => {
                        let lines = c
                            .data
                            .question
                            .html_to_text()
                            .lines()
                            .map(|l| l.to_string())
                            .collect::<Vec<String>>();
                        emit!(Popup(lines));
                    }
                    Err(e) => {
                        emit!(Error(e.to_string()));
                    }
                };
            });
        } else {
            log::debug!("hovered question is none");
        }
        false
    }

    pub fn run_solution(&self) -> bool {
        if let Some(_hovered) = self.hovered() {
            let id = _hovered.id.id.to_string();
            if let Ok(lang_refs) = SOLUTION_FILE_MANAGER
                .get()
                .unwrap()
                .read()
                .unwrap()
                .get_available_languages(id.as_str())
                .emit()
            {
                let cloned_langs = lang_refs.iter().map(|v| v.to_string()).collect();
                tokio::spawn(async move {
                    if let Some(selected_lang) = emit!(SelectPopup(cloned_langs)).await {
                        let selected_sol_file = SOLUTION_FILE_MANAGER
                            .get()
                            .unwrap()
                            .read()
                            .unwrap()
                            .get_solution_file(id.as_str(), selected_lang)
                            .cloned();
                        if let Ok(_f) = selected_sol_file.emit() {
                            let contents = _f.read_contents().await.unwrap();
                            let lang = _f.language;
                            if let Ok(response) =
                                RunCodeRequest::new(lang, _f.question_id, contents, _f.title_slug)
                                    .poll_check_response(REQ_CLIENT.as_ref())
                                    .await
                                    .emit()
                            {
                                emit!(Popup(vec![response.to_string()]));
                            }
                        }
                    }
                });
            }
        }
        false
    }

    pub fn select_language(&self) -> bool {
        if let Some(_hovered) = self.hovered() {
            let slug = _hovered.title_slug.clone();
            tokio::spawn(async move {
                let editor_data = leetcode_core::EditorDataRequest::new(slug)
                    .send(REQ_CLIENT.as_ref())
                    .await;
                match editor_data {
                    Ok(ed) => {
                        if let Some(selected) = emit!(SelectPopup(
                            ed.get_languages().iter().map(|l| l.to_string()).collect()
                        ))
                        .await
                        {
                            let selected_lang = ed.get_languages()[selected];
                            let editor_data = ed.get_editor_data_by_language(selected_lang);
                            let file_name = ed.get_filename(selected_lang);
                            match file_name {
                                Ok(f_name) => {
                                    if let Some(e_data) = editor_data {
                                        match SOLUTION_FILE_MANAGER
                                            .get()
                                            .unwrap()
                                            .write()
                                            .unwrap()
                                            .create_solution_file(f_name.as_str(), e_data)
                                        {
                                            Ok(written_path) => {
                                                emit!(Open(written_path));
                                            }
                                            Err(e) => {
                                                let err_msg =  format!(
                                                    "Could not write to the solution directory {e} question: {:?}, lang: {:?}",
                                                     ed.data.question.title_slug,
                                                     selected_lang
                                                );
                                                emit!(Error(err_msg));
                                            }
                                        };
                                    } else {
                                        emit!(Error(format!(
                                            "Editor data not found for question: {:?}, lang: {:?}",
                                            ed.data.question.title_slug, selected_lang,
                                        )));
                                    }
                                }
                                Err(e) => {
                                    emit!(Error(e.to_string()));
                                }
                            }
                        } else {
                            log::info!("quitting popup unselected");
                        }
                    }
                    Err(e) => {
                        emit!(Error(e.to_string()));
                    }
                }
            });
        }
        false
    }

    pub fn set_questions(&mut self, questions: Vec<DbQuestion>) {
        self.paginate.update_list(questions)
    }
}
