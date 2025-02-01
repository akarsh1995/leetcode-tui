pub(super) mod sol_dir;
mod stats;
use crate::SendError;
use crate::{emit, utils::Paginate};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use html2md::parse_html;
use leetcode_core::graphql::query::{daily_coding_challenge, RunOrSubmitCodeCheckResult};
use leetcode_core::types::run_submit_response::display::CustomDisplay;
use leetcode_core::types::run_submit_response::ParsedResponse;
use leetcode_core::{
    GQLLeetcodeRequest, QuestionContentRequest, RunCodeRequest, SubmitCodeRequest,
};
use leetcode_tui_config::log;
use leetcode_tui_db::{DbQuestion, DbTopic};
use leetcode_tui_shared::layout::Window;
pub(crate) use sol_dir::init;
use sol_dir::SOLUTION_FILE_MANAGER;
use stats::Stats;
use std::rc::Rc;

pub struct Questions {
    paginate: Paginate<Rc<DbQuestion>>,
    ques_haystack: Vec<Rc<DbQuestion>>,
    needle: Option<String>,
    matcher: SkimMatcherV2,
    show_stats: bool,
}

impl Default for Questions {
    fn default() -> Self {
        Self {
            paginate: Paginate::new(vec![]),
            needle: Default::default(),
            ques_haystack: vec![],
            matcher: Default::default(),
            show_stats: Default::default(),
        }
    }
}

impl Questions {
    pub fn prev_ques(&mut self) -> bool {
        self.paginate.prev_elem(self.widget_height())
    }

    pub fn next_ques(&mut self) -> bool {
        self.paginate.next_elem(self.widget_height())
    }

    pub fn rand_ques(&mut self) -> bool {
        self.paginate.rand_elem(self.widget_height())
    }

    pub fn window(&self) -> &[Rc<DbQuestion>] {
        self.paginate.window(self.widget_height())
    }

    pub fn hovered(&self) -> Option<&Rc<DbQuestion>> {
        self.paginate.hovered()
    }

    pub fn set_adhoc(&mut self, question: DbQuestion) -> bool {
        if let Some(id) = self.ques_haystack.iter().position(|x| x.id == question.id) {
            self.needle = None;
            self.filter_questions();
            self.paginate.set_element_by_index(id, self.widget_height());
            return true;
        } else {
            emit!(Popup(
                "not",
                vec![format!(
                    "Question not found with id={}, title={}",
                    question.id, question.title
                )]
            ));
        };
        return false;
    }

    fn widget_height(&self) -> usize {
        let window = Window::default();
        let height = window.root.center_layout.question.inner.height;
        height as usize
    }
}

impl Questions {
    async fn get_question_content(slug: &str) -> Vec<String> {
        let qc = QuestionContentRequest::new(slug.to_string());
        if let Ok(content) = qc.send().await.emit_if_error() {
            let lines = content
                .data
                .question
                .html_to_text()
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>();
            return lines;
        }
        return vec!["".into()];
    }

    pub fn show_question_content(&self) -> bool {
        if let Some(_hovered) = self.hovered() {
            let slug = _hovered.title_slug.clone();
            let title = _hovered.title.clone();
            tokio::spawn(async move {
                let lines = Self::get_question_content(slug.as_str()).await;
                emit!(Popup(title, lines));
            });
        } else {
            log::debug!("hovered question is none");
        }
        true
    }

    pub fn run_solution(&self) -> bool {
        self._run_solution(false)
    }

    pub fn submit_solution(&self) -> bool {
        self._run_solution(true)
    }

    fn _run_solution(&self, is_submit: bool) -> bool {
        if let Some(_hovered) = self.hovered() {
            let mut cloned_quest = _hovered.as_ref().clone();
            let id = _hovered.id.to_string();
            if let Ok(lang_refs) = SOLUTION_FILE_MANAGER
                .get()
                .unwrap()
                .read()
                .unwrap()
                .get_available_languages(id.as_str())
                .emit_if_error()
            {
                let cloned_langs = lang_refs.iter().map(|v| v.to_string()).collect();
                tokio::spawn(async move {
                    if let Some(selected_lang) =
                        emit!(SelectPopup("Available solutions in", cloned_langs)).await
                    {
                        let selected_sol_file = SOLUTION_FILE_MANAGER
                            .get()
                            .unwrap()
                            .read()
                            .unwrap()
                            .get_solution_file(id.as_str(), selected_lang)
                            .cloned();
                        if let Ok(f) = selected_sol_file.emit_if_error() {
                            if let Ok(contents) = f.read_contents().await.emit_if_error() {
                                let lang = f.language;
                                let request = if is_submit {
                                    SubmitCodeRequest::new(
                                        lang,
                                        f.question_id,
                                        contents,
                                        f.title_slug,
                                    )
                                    .poll_check_response()
                                    .await
                                } else {
                                    let mut run_code_req = RunCodeRequest::new(
                                        lang,
                                        None,
                                        f.question_id,
                                        contents,
                                        f.title_slug,
                                    );
                                    if let Err(e) = run_code_req
                                        .set_sample_test_cases_if_none()
                                        .await
                                        .emit_if_error()
                                    {
                                        log::info!(
                                            "error while setting the sample testcase list {}",
                                            e
                                        );
                                        return;
                                    } else {
                                        run_code_req.poll_check_response().await
                                    }
                                };

                                if let Ok(response) = request.emit_if_error() {
                                    if let Ok(update_result) =
                                        cloned_quest.mark_attempted().emit_if_error()
                                    {
                                        // when solution is just run against sample cases
                                        if update_result.is_some() {
                                            // fetches latest result from db
                                            emit!(QuestionUpdate);
                                        }
                                    }

                                    if is_submit {
                                        let is_submission_accepted =
                                            matches!(response, ParsedResponse::SubmitAccepted(..));
                                        if is_submission_accepted {
                                            if let Ok(update_result) =
                                                cloned_quest.mark_accepted().emit_if_error()
                                            {
                                                // when solution is accepted
                                                if update_result.is_some() {
                                                    // fetches latest result from db
                                                    emit!(QuestionUpdate);
                                                }
                                            };
                                        }
                                    }
                                    emit!(Popup(response.get_display_lines()));
                                }
                            }
                        }
                    }
                });
            }
        }
        false
    }

    pub fn solve_for_language(&self) -> bool {
        if let Some(_hovered) = self.hovered() {
            let slug = _hovered.title_slug.clone();
            tokio::spawn(async move {
                if let Ok(editor_data) = leetcode_core::EditorDataRequest::new(slug)
                    .send()
                    .await
                    .emit_if_error()
                {
                    if let Some(selected) = emit!(SelectPopup(
                        "Select Language",
                        editor_data
                            .get_languages()
                            .iter()
                            .map(|l| l.to_string())
                            .collect()
                    ))
                    .await
                    {
                        let selected_lang = editor_data.get_languages()[selected];
                        let editor_content = editor_data.get_editor_data_by_language(selected_lang);
                        let question_content = editor_data.data.question.content.as_str();

                        if let Ok(file_name) =
                            editor_data.get_filename(selected_lang).emit_if_error()
                        {
                            if let Some(e_data) = editor_content {
                                let file_contents = format!(
                                    "{}\n\n\n{}",
                                    selected_lang.comment_text(&parse_html(question_content)),
                                    e_data
                                );
                                if let Ok(written_path) = SOLUTION_FILE_MANAGER
                                    .get()
                                    .unwrap()
                                    .write()
                                    .unwrap()
                                    .create_solution_file(
                                        file_name.as_str(),
                                        file_contents.as_str(),
                                    )
                                    .emit_if_error()
                                {
                                    emit!(Open(written_path));
                                }
                            };
                        };
                    } else {
                        log::info!("quitting popup unselected");
                    }
                }
            });
        }
        false
    }

    pub fn set_questions(&mut self, questions: Vec<DbQuestion>) {
        self.ques_haystack = questions.into_iter().map(Rc::new).collect();
        self.filter_questions();
    }

    pub fn add_question(&mut self, question: DbQuestion) {
        self.ques_haystack.push(Rc::new(question));
    }

    pub fn toggle_daily_question(&self) -> bool {
        tokio::spawn(async move {
            let daily_challenge_question = daily_coding_challenge::Query::new()
                .send()
                .await
                .emit_if_error()
                .unwrap();

            let mut db_question: DbQuestion = daily_challenge_question
                .data
                .active_daily_coding_challenge_question
                .question
                .try_into()
                .emit_if_error()
                .unwrap();

            db_question.save_to_db().unwrap();
            emit!(Topic(DbTopic { slug: "all".into() }));
            emit!(AdhocQuestion(db_question));
        });
        false
    }
}

impl Questions {
    pub fn toggle_search(&mut self) -> bool {
        let existing_needle = self.needle.clone();
        tokio::spawn(async move {
            let mut rx = emit!(Input(existing_needle));
            while let Some(maybe_needle) = rx.recv().await {
                if let Some(needle) = maybe_needle {
                    emit!(QuestionFilter(Some(needle)));
                } else {
                    break;
                }
            }
        });
        false
    }

    pub fn filter_by(&mut self, string: Option<String>) {
        if self.needle != string {
            self.needle = string;
            self.filter_questions();
        }
    }

    fn filter_questions(&mut self) {
        self.ques_haystack.sort();
        let fil_quests = if let Some(needle) = self.needle.as_ref() {
            let quests: Vec<Rc<DbQuestion>> = self
                .ques_haystack
                .iter()
                .filter(|q| {
                    let search_string = format!(
                        "{} {} {}", // id, topics, title
                        q.id,
                        q.topics
                            .iter()
                            .map(|t| t.slug.as_str())
                            .collect::<Vec<&str>>()
                            .join(", "),
                        q.title
                    );

                    self.matcher
                        .fuzzy_match(search_string.as_str(), &needle)
                        .is_some()
                })
                .cloned()
                .collect();
            quests
        } else {
            self.ques_haystack.clone()
        };
        self.paginate.update_list(fil_quests);
    }
}

impl Questions {
    pub fn get_stats(&self) -> Stats<'_> {
        Stats::new(&self.ques_haystack)
    }

    pub fn toggle_stats(&mut self) -> bool {
        self.show_stats = !self.show_stats;
        true
    }

    pub fn is_stats_visible(&self) -> bool {
        self.show_stats
    }
}
