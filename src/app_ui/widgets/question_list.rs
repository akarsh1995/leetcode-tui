pub(crate) mod custom_lists;
mod tasks;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::app_ui::async_task_channel::TaskRequest::DbUpdateQuestion;
use crate::app_ui::async_task_channel::{Request, Response, TaskResponse};
use crate::app_ui::components::help_text::{CommonHelpText, HelpText};
use crate::app_ui::components::popups::paragraph::ParagraphPopup;
use crate::app_ui::components::popups::selection_list::SelectionListPopup;
use crate::app_ui::event::VimPingSender;
use crate::app_ui::helpers::matcher::Matcher;
use crate::app_ui::helpers::utils::{generate_random_string, SolutionFile};
use crate::app_ui::{async_task_channel::ChannelRequestSender, components::list::StatefulList};
use crate::config::Config;
use crate::deserializers;
use crate::deserializers::editor_data::CodeSnippet;
use crate::deserializers::run_submit::{ParsedResponse, Success};
use crate::entities::{QuestionModel, TopicTagModel};
use crate::errors::{AppResult, LcAppError};
use crate::graphql::run_code::RunCode;
use crate::graphql::submit_code::SubmitCode;
use crate::graphql::{Language, RunOrSubmitCode};

use crossterm::event::{KeyCode, KeyEvent};
use indexmap::{IndexMap, IndexSet};
use ratatui::widgets::Paragraph;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use self::custom_lists::NEETCODE_75;
use self::tasks::{
    process_get_all_question_map_task_content, process_question_detail_response,
    process_question_editor_data,
};

use super::notification::{NotifContent, Notification, PopupMessage, PopupType, WidgetName};
use super::{CommonState, CrosstermStderr, Widget};
use crate::app_ui::components::color::{Callout, TokyoNightColors, CHECK_MARK};
use lru;
use std::num::NonZeroUsize;
use tasks::TaskType;

#[derive(Debug, Default)]
struct CachedQuestion {
    editor_data: Option<deserializers::editor_data::Question>,
    qd: Option<deserializers::question_content::QuestionContent>,
}

impl CachedQuestion {
    fn question_data_received(&self) -> bool {
        self.qd.is_some()
    }

    fn editor_data_received(&self) -> bool {
        self.editor_data.is_some()
    }

    fn get_code_snippets(&self) -> Option<&Vec<CodeSnippet>> {
        if let Some(ed) = &self.editor_data {
            return Some(&ed.code_snippets);
        }
        None
    }

    fn get_list_of_languages(&self) -> Option<Vec<String>> {
        if let Some(cs) = self.get_code_snippets() {
            return Some(cs.iter().map(|s| s.lang.clone()).collect());
        }
        None
    }

    fn get_question_content(&self) -> Option<String> {
        if let Some(content) = &self.qd {
            return Some(content.html_to_text());
        }
        None
    }
}

#[derive(Debug)]
enum State {
    JumpingTo,
    Filter,
    Normal,
}

type Question = Rc<RefCell<QuestionModel>>;

#[derive(Debug)]
pub struct QuestionListWidget {
    pub common_state: CommonState,
    pub questions: StatefulList<Question>,
    pub topic_tag_question_map: HashMap<Rc<TopicTagModel>, Vec<Question>>,
    vim_tx: VimPingSender,
    vim_running: Arc<AtomicBool>,

    // (frontend_question_id, CachedQuestion)
    cache: lru::LruCache<String, CachedQuestion>,

    // (random_request_id, task type)
    task_map: HashMap<String, (Question, TaskType)>,

    // (keyevent, frontend_question_id)
    pending_event_actions: IndexSet<(KeyEvent, String)>,

    // (frontend_question_id, Rc<RefCell<QuestionModel>>)
    _fid_question_mapping: IndexMap<String, Question>,

    config: Rc<Config>,

    // (frontend_question_id, MultipleLangSolutionsSet)
    files: HashMap<i32, HashSet<SolutionFile>>,
    jump_to: usize,
    state: State,
    selected_topic: Option<TopicTagModel>,
    needle: Option<String>,
}

impl QuestionListWidget {
    pub fn new(
        id: WidgetName,
        task_sender: ChannelRequestSender,
        vim_tx: VimPingSender,
        vim_running: Arc<AtomicBool>,
        config: Rc<Config>,
    ) -> Self {
        let mut files: HashMap<i32, HashSet<SolutionFile>> = HashMap::new();
        for file in config
            .solutions_dir
            .read_dir()
            .expect("Cannot read the solutions directory")
            .flatten()
        {
            if file.path().is_file() {
                if let Some(sf) = SolutionFile::from_file(file.path()) {
                    let qid = sf
                        .question_id
                        .clone()
                        .parse::<i32>()
                        .expect("frontend_question_id is not a number");
                    files.entry(qid).or_default().insert(sf);
                }
            }
        }
        Self {
            common_state: CommonState::new(
                id,
                task_sender,
                vec![
                    CommonHelpText::SwitchPane.into(),
                    CommonHelpText::ScrollUp.into(),
                    CommonHelpText::ScrollDown.into(),
                    CommonHelpText::Edit.into(),
                    CommonHelpText::ReadContent.into(),
                    CommonHelpText::Run.into(),
                    CommonHelpText::Submit.into(),
                ],
            ),
            topic_tag_question_map: HashMap::new(),
            questions: Default::default(),
            vim_tx,
            vim_running,
            cache: lru::LruCache::new(NonZeroUsize::new(10).unwrap()),
            task_map: HashMap::new(),
            pending_event_actions: Default::default(),
            config,
            files,
            jump_to: 0,
            state: State::Normal,
            _fid_question_mapping: IndexMap::new(),
            needle: None,
            selected_topic: None,
        }
    }
}

impl QuestionListWidget {
    fn peek_cache_by_question(&mut self, question: &Question) -> Option<&CachedQuestion> {
        self.cache.peek(&question.borrow().frontend_question_id)
    }

    fn send_fetch_question_editor_details(&mut self, question: Question) -> AppResult<()> {
        if let Some(cached_q) = self.peek_cache_by_question(&question) {
            if !cached_q.question_data_received() {
                self.send_fetch_question_details(question.clone())?;
            }
        }
        self.show_spinner()?;
        let random_key = generate_random_string(10);
        self.task_map
            .insert(random_key.clone(), (question.clone(), TaskType::Edit));
        self.get_task_sender()
            .send(
                crate::app_ui::async_task_channel::TaskRequest::GetQuestionEditorData(Request {
                    widget_name: self.get_widget_name(),
                    request_id: random_key,
                    content: question.borrow().title_slug.clone(),
                }),
            )
            .map_err(Box::new)?;
        Ok(())
    }

    fn send_fetch_solution_run_details(
        &mut self,
        question: Question,
        lang: Language,
        typed_code: String,
        is_submit: bool,
    ) -> AppResult<()> {
        self.show_spinner()?;
        let random_key = generate_random_string(10);
        self.task_map
            .insert(random_key.clone(), (question.clone(), TaskType::Run));

        let content = if is_submit {
            let submit_code = SubmitCode {
                lang,
                question_id: question.borrow().frontend_question_id.clone(),
                typed_code,
                slug: question.borrow().title_slug.clone(),
            };

            RunOrSubmitCode::Submit(submit_code)
        } else {
            let run_code = RunCode {
                lang,
                question_id: question.borrow().frontend_question_id.clone(),
                typed_code,
                test_cases_stdin: None, // automatically fetches sample test cases from the server
                slug: question.borrow().title_slug.clone(),
            };

            RunOrSubmitCode::Run(run_code)
        };

        self.get_task_sender()
            .send(
                crate::app_ui::async_task_channel::TaskRequest::CodeRunRequest(Request {
                    widget_name: self.get_widget_name(),
                    request_id: random_key,
                    content,
                }),
            )
            .map_err(Box::new)?;
        Ok(())
    }

    fn solution_file_popup_action(
        &mut self,
        question: Question,
        task_type: TaskType,
        index: usize,
    ) -> AppResult<()> {
        self.show_spinner()?;
        let solution_files = self
            .files
            .get(
                &question
                    .borrow()
                    .frontend_question_id
                    .clone()
                    .parse()
                    .unwrap(),
            )
            .expect("Question id does not exist in the solutions mapping");
        let solution_file = solution_files.iter().nth(index).unwrap();
        let typed_code = solution_file.read_file_contents(&self.config.solutions_dir);
        let is_submit = match task_type {
            TaskType::Run => false,
            TaskType::Submit => true,
            _ => unimplemented!(),
        };
        self.send_fetch_solution_run_details(
            question,
            solution_file.lang.clone(),
            typed_code,
            is_submit,
        )
    }

    fn send_fetch_question_details(&mut self, question: Question) -> AppResult<()> {
        self.show_spinner()?;
        let random_key = generate_random_string(10);
        self.task_map
            .insert(random_key.clone(), (question.clone(), TaskType::Read));
        self.get_task_sender()
            .send(
                crate::app_ui::async_task_channel::TaskRequest::QuestionDetail(Request {
                    widget_name: self.get_widget_name(),
                    request_id: random_key,
                    content: question.borrow().title_slug.clone(),
                }),
            )
            .map_err(Box::new)?;
        Ok(())
    }

    fn sync_db_solution_submit_status(&mut self, question: &Question) -> AppResult<()> {
        self.show_spinner()?;
        self.get_task_sender()
            .send(DbUpdateQuestion(Request {
                widget_name: self.get_widget_name(),
                request_id: "".to_string(),
                content: question.borrow().to_owned(),
            }))
            .map_err(Box::new)?;
        Ok(())
    }

    fn is_notif_pending(&self, key: &(KeyEvent, Question)) -> bool {
        self.pending_event_actions
            .contains(&(key.0, key.1.borrow().frontend_question_id.clone()))
    }

    fn open_vim_like_editor(&mut self, file_name: &Path, editor: &str) -> AppResult<()> {
        let mut output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&format!("{} {}", editor, file_name.display()))
            .spawn()
            .map_err(|e| LcAppError::EditorOpen(format!("Can't spawn {} editor: {e}", editor)))?;
        self.vim_running
            .store(true, std::sync::atomic::Ordering::Relaxed);
        let vim_cmd_result = output
            .wait()
            .map_err(|e| LcAppError::EditorOpen(format!("Editor Error: {e}")))?;
        self.vim_running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.vim_tx.blocking_send(1).unwrap();
        if !vim_cmd_result.success() {
            return Err(LcAppError::EditorOpen(
                "Cannot open editor, Reason: Unknown".to_string(),
            ));
        }
        Ok(())
    }

    fn open_editor(&mut self, file_name: &Path) -> AppResult<()> {
        if let Ok(editor) = std::env::var("EDITOR") {
            if editor.contains("vim") || editor.contains("nano") {
                self.open_vim_like_editor(file_name, editor.as_str())?;
            } else {
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&format!("{} {}", editor, file_name.display()))
                    .spawn()?
                    .wait()?;
            }
        } else {
            // try open vim
            self.open_vim_like_editor(file_name, "vim")?;
        }
        Ok(())
    }

    fn popup_list_notification(
        &mut self,
        popup_content: Vec<String>,
        question_title: String,
        popup_key: String,
        help_texts: IndexSet<HelpText>,
    ) -> Notification {
        Notification::Popup(NotifContent {
            src_wid: self.get_widget_name(),
            dest_wid: WidgetName::Popup,
            content: PopupMessage {
                help_texts,
                popup: PopupType::List {
                    popup: SelectionListPopup::new(question_title, popup_content),
                    key: popup_key,
                },
            },
        })
    }

    fn popup_paragraph_notification(
        &self,
        popup_content: String,
        popup_title: String,
        help_texts: IndexSet<HelpText>,
    ) -> Notification {
        Notification::Popup(NotifContent {
            src_wid: self.get_widget_name(),
            dest_wid: WidgetName::Popup,
            content: PopupMessage {
                help_texts,
                popup: PopupType::Paragraph(ParagraphPopup::new(popup_title, popup_content)),
            },
        })
    }

    fn get_question_list_render_item(question: &Question) -> ListItem {
        let number = &question.borrow().frontend_question_id;
        let title = &question.borrow().title;

        let is_accepted = question
            .borrow()
            .status
            .as_ref()
            .map_or(false, |v| v.as_str() == "ac");

        let line_text = format!(
            "{} {:0>3}: {}",
            {
                if is_accepted {
                    CHECK_MARK
                } else {
                    " "
                }
            },
            number,
            title
        );

        let qs_diff = question.borrow().difficulty.clone();

        let text_color = match qs_diff.as_str() {
            "Easy" => Callout::Success,
            "Medium" => Callout::Warning,
            "Hard" => Callout::Error,
            "Disabled" => Callout::Disabled,
            _ => unimplemented!(),
        };

        let styled_title = Span::styled(line_text, text_color.into());
        ListItem::new(styled_title)
    }

    fn add_event_to_event_queue(&mut self, data: (KeyEvent, String)) -> bool {
        self.pending_event_actions.insert(data)
    }

    fn process_pending_events(&mut self) {
        let mut to_process_again = vec![];
        while let Some((pending_event, question_id)) = self.pending_event_actions.pop() {
            let question_model = self._fid_question_mapping[&question_id].clone();
            let ques_in_cache = self
                .cache
                .get_or_insert_mut(question_id.clone(), CachedQuestion::default);
            match pending_event.code {
                KeyCode::Enter => {
                    if let Some(cache_ques) = &ques_in_cache.qd {
                        let content = cache_ques.html_to_text();
                        let title = question_model.borrow().title.clone();
                        let notif = self.popup_paragraph_notification(
                            content,
                            title,
                            IndexSet::from_iter([CommonHelpText::Edit.into()]),
                        );
                        self.get_notification_queue().push_back(notif);
                    } else {
                        to_process_again.push((pending_event, question_id));
                    }
                }
                KeyCode::Char('E') | KeyCode::Char('e') => {
                    let question_data_in_cache = ques_in_cache.question_data_received();
                    let question_editor_data_in_cache = ques_in_cache.editor_data_received();

                    if question_data_in_cache && question_editor_data_in_cache {
                        let content = ques_in_cache.get_list_of_languages().unwrap();
                        let title = "Select Language".to_string();
                        let popup_key = generate_random_string(10);
                        self.task_map
                            .insert(popup_key.clone(), (question_model.clone(), TaskType::Edit));
                        let notif = self.popup_list_notification(
                            content,
                            title,
                            popup_key,
                            IndexSet::new(),
                        );
                        self.get_notification_queue().push_back(notif);
                    } else {
                        to_process_again.push((pending_event, question_id));
                    }
                }
                _ => continue,
            }
        }

        for i in to_process_again {
            self.add_event_to_event_queue(i);
        }
    }

    fn get_selected_question_from_cache(&mut self) -> (&mut CachedQuestion, Question) {
        let selected_question = self.questions.get_selected_item();
        let sel = selected_question.expect("no question selected");
        let model = sel.clone();
        let k = self.cache.get_or_insert_mut(
            model.borrow().frontend_question_id.clone(),
            CachedQuestion::default,
        );
        (k, model)
    }

    fn run_or_submit_code_event_handler(
        &mut self,
        task_type: TaskType,
    ) -> AppResult<Option<Notification>> {
        let selected_question = self
            .questions
            .get_selected_item()
            .expect("no question selected");
        let id: i32 = selected_question
            .borrow()
            .frontend_question_id
            .parse()
            .unwrap();
        if let Some(files) = self.files.get(&id) {
            let langs = files
                .iter()
                .map(|f| f.lang.clone().to_string())
                .collect::<Vec<_>>();
            let key = generate_random_string(10);
            self.task_map
                .insert(key.clone(), (selected_question.clone(), task_type));
            return Ok(Some(self.popup_list_notification(
                langs,
                "Select Language".to_string(),
                key,
                IndexSet::new(),
            )));
        }
        Ok(Some(self.popup_paragraph_notification(
            "Kindly press key 'e' to create the solution file first.".to_string(),
            "Help".to_string(),
            IndexSet::new(),
        )))
    }

    fn process_neetcode_75_questions(&mut self) {
        self.topic_tag_question_map.insert(
            Rc::new(custom_lists::NEETCODE_75.get_topic_tag()),
            NEETCODE_75.filter_questions(self._fid_question_mapping.values()),
        );
    }

    fn is_selected_topic_all(&self) -> bool {
        if let Some(st) = &self.selected_topic {
            return st.id == "all";
        }
        false
    }

    fn update_questions_based_on_filter(&mut self) {
        if let Some(needle) = &self.needle {
            if let Some(selected_topic) = &self.selected_topic {
                let j = &self.topic_tag_question_map[selected_topic]
                    .iter()
                    .map(|q| q.borrow())
                    .collect::<Vec<_>>();
                let question_strs = j.iter().map(|q| q.title.as_str());
                let mut m = Matcher::new(Some(question_strs));
                if let Some(matching_indices) = m.match_with_key(needle.as_str()) {
                    let matches: HashSet<usize> = HashSet::from_iter(matching_indices);
                    self.questions.items = self.topic_tag_question_map[selected_topic]
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| matches.contains(i))
                        .map(|(_, q)| q.clone())
                        .collect()
                }
            }
        }
    }
}

impl super::Widget for QuestionListWidget {
    fn parent_can_handle_events(&self) -> bool {
        matches!(self.state, State::Normal)
    }

    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let mut question_list_chunk = rect;
        if matches!(self.state, State::Filter) {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(95), Constraint::Percentage(5)].as_ref())
                .split(rect);
            let (question_chunk, search_chunk) = (chunks[0], chunks[1]);
            let needle = self.needle.as_ref().map_or("", |v| v.as_str());
            let search_color: ratatui::style::Color = TokyoNightColors::Pink.into();
            let text_color: ratatui::style::Color = TokyoNightColors::Foreground.into();
            let search_bar = Line::from(vec![
                Span::from("Search: ").fg(search_color),
                Span::from(needle).fg(text_color),
            ]);
            let p = Paragraph::new(search_bar).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(TokyoNightColors::Pink.into())),
            );
            frame.render_widget(p, search_chunk);
            question_list_chunk = question_chunk;
        }

        let lines = self
            .questions
            .items
            .iter()
            .map(Self::get_question_list_render_item)
            .collect::<Vec<_>>();

        let mut border_style = Style::default();
        if self.is_active() {
            border_style = border_style.fg(TokyoNightColors::Pink.into());
        }

        let items = List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Questions")
                    .border_style(border_style),
            )
            .highlight_style(
                Style::default()
                    .bg(TokyoNightColors::Selection.into())
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_stateful_widget(items, question_list_chunk, &mut self.questions.state);
    }

    fn handler(&mut self, event: KeyEvent) -> AppResult<Option<Notification>> {
        match (&self.state, event.code) {
            (_, KeyCode::Esc) => {
                self.state = State::Normal;
            }
            (State::Normal, e) => match e {
                KeyCode::Up => self.questions.previous(),
                KeyCode::Down => self.questions.next(),
                KeyCode::Enter => {
                    let (cache, model) = self.get_selected_question_from_cache();
                    let question_data_in_cache = cache.question_data_received();

                    if question_data_in_cache {
                        let content = cache.get_question_content().unwrap();
                        let title = model.borrow().title.clone();
                        return Ok(Some(self.popup_paragraph_notification(
                            content,
                            title,
                            IndexSet::from_iter([CommonHelpText::Edit.into()]),
                        )));
                    }

                    if self.is_notif_pending(&(event, model.clone())) {
                        self.process_pending_events();
                        return Ok(None);
                    }

                    // before sending the task request set the key as the request id and value
                    // as question model so that we can obtain the question model once we get the response
                    self.send_fetch_question_details(model.clone())?;
                    self.add_event_to_event_queue((
                        event,
                        model.borrow().frontend_question_id.clone(),
                    ));
                }

                KeyCode::Char('e') | KeyCode::Char('E') => {
                    let (cache, model) = self.get_selected_question_from_cache();
                    let question_data_in_cache = cache.question_data_received();
                    let question_editor_data_in_cache = cache.editor_data_received();

                    if question_data_in_cache && question_editor_data_in_cache {
                        let content = cache.get_list_of_languages().unwrap();
                        let title = "Select Language".to_string();
                        let popup_key = generate_random_string(10);
                        self.task_map
                            .insert(popup_key.clone(), (model, TaskType::Edit));
                        let notif = self.popup_list_notification(
                            content,
                            title,
                            popup_key,
                            IndexSet::new(),
                        );
                        return Ok(Some(notif));
                    }

                    if self.is_notif_pending(&(event, model.clone())) {
                        self.process_pending_events();
                        return Ok(None);
                    }

                    // before sending the task request set the key as the request id and value
                    // as question model so that we can obtain the question model once we get the response
                    self.send_fetch_question_editor_details(model.clone())?;
                    self.add_event_to_event_queue((
                        event,
                        model.borrow().frontend_question_id.clone(),
                    ));
                }

                KeyCode::Char('r') | KeyCode::Char('R') => {
                    return self.run_or_submit_code_event_handler(TaskType::Run);
                }
                KeyCode::Char('s') => {
                    return self.run_or_submit_code_event_handler(TaskType::Submit);
                }
                KeyCode::Char(c) => {
                    if c.is_numeric() {
                        self.state = State::JumpingTo;
                        self.jump_to = 0;
                        let digit = c.to_digit(10).unwrap() as usize;
                        self.jump_to *= 10;
                        self.jump_to += digit;
                    }
                    if c == '/' {
                        self.state = State::Filter;
                    }
                }
                _ => {}
            },
            (State::JumpingTo, e) => {
                if let KeyCode::Char(c) = e {
                    if c.is_numeric() {
                        let digit = c.to_digit(10).unwrap() as usize;
                        self.jump_to *= 10;
                        self.jump_to += digit;
                    } else if c == 'G' {
                        if !self.is_selected_topic_all() {
                            self.state = State::Normal;
                            self.jump_to = 0;
                            return Ok(Some(self.popup_paragraph_notification(
                                "Can only use jump to in all topic section".to_string(),
                                "Jump Info".to_string(),
                                IndexSet::new(),
                            )));
                        }
                        let mut failed_notif_msg = None;
                        if self.jump_to > self.questions.items.len() {
                            failed_notif_msg = Some(format!(
                                "Max range {}. You entered {}.",
                                self.questions.items.len(),
                                self.jump_to
                            ));
                        } else if self.jump_to != 0 {
                            self.questions.state.select(Some(self.jump_to - 1));
                        } else if self.jump_to == 0 {
                            failed_notif_msg = Some("No Question with id = 0".to_string());
                        }
                        self.state = State::Normal;
                        self.jump_to = 0;
                        return Ok(failed_notif_msg.map(|msg| {
                            self.popup_paragraph_notification(
                                msg,
                                "Jump failed".to_string(),
                                IndexSet::new(),
                            )
                        }));
                    } else {
                        self.state = State::Normal;
                        self.jump_to = 0;
                    }
                }
            }
            (State::Filter, keycode) => match keycode {
                KeyCode::Char(c) => {
                    if let Some(n) = &mut self.needle {
                        n.push(c)
                    } else {
                        self.needle = Some(c.to_string())
                    }
                    self.update_questions_based_on_filter();
                }
                KeyCode::Backspace => {
                    if let Some(s) = self.needle.as_mut() {
                        if !s.is_empty() {
                            s.pop();
                        } else {
                            self.needle = None;
                            self.state = State::Normal;
                        }
                        self.update_questions_based_on_filter();
                    }
                }
                _ => {
                    self.state = State::Normal;
                    return self.handler(event);
                }
            },
        };
        Ok(None)
    }

    fn setup(&mut self) -> AppResult<()> {
        self.get_task_sender()
            .send(
                crate::app_ui::async_task_channel::TaskRequest::GetAllQuestionsMap(Request {
                    widget_name: self.get_widget_name(),
                    request_id: "".to_string(),
                    content: (),
                }),
            )
            .map_err(Box::new)?;
        Ok(())
    }

    fn process_task_response(&mut self, response: TaskResponse) -> AppResult<()> {
        match response {
            TaskResponse::GetAllQuestionsMap(Response { content, .. }) => {
                process_get_all_question_map_task_content(
                    content,
                    &mut self.topic_tag_question_map,
                    &mut self._fid_question_mapping,
                );

                self.process_neetcode_75_questions();

                self.get_notification_queue()
                    .push_back(Notification::Questions(NotifContent::new(
                        WidgetName::QuestionList,
                        super::notification::WidgetName::QuestionList,
                        vec![TopicTagModel {
                            name: "All".to_owned(),
                            id: "all".to_owned(),
                            slug: "all".to_owned(),
                        }],
                    )));
            }
            TaskResponse::QuestionDetail(qd) => {
                process_question_detail_response(qd, &mut self.task_map, &mut self.cache);
            }
            TaskResponse::QuestionEditorData(ed) => {
                process_question_editor_data(ed, &mut self.task_map, &mut self.cache);
            }
            TaskResponse::RunResponseData(run_res) => {
                let popup_content = run_res.content.to_string();
                let mut is_submit = false;
                if matches!(
                    run_res.content,
                    ParsedResponse::Success(Success::Submit { .. })
                ) {
                    is_submit = true;
                    // upon successful submit of the question update the question accepted status
                    // also update the db
                    {
                        let (model, _) = &self.task_map[&run_res.request_id];
                        model.borrow_mut().status = Some("ac".to_string());
                        self.sync_db_solution_submit_status(&model.clone())?;
                    }
                }
                let notification = self.popup_paragraph_notification(
                    popup_content,
                    format!("{} Status", (if is_submit { "Submit" } else { "Run" })),
                    IndexSet::new(),
                );
                // post submit remove the reference_task_key from task_map
                self.get_notification_queue().push_back(notification);
                self.task_map.remove(&run_res.request_id).unwrap();
            }
            TaskResponse::Error(e) => {
                let src_wid = self.get_widget_name();
                self.get_notification_queue()
                    .push_back(Notification::Popup(NotifContent {
                        src_wid,
                        dest_wid: WidgetName::Popup,
                        content: PopupMessage {
                            help_texts: IndexSet::new(),
                            popup: PopupType::Paragraph(ParagraphPopup::new(
                                "Error Encountered".into(),
                                e.content,
                            )),
                        },
                    }));
            }
            _ => {}
        }
        self.hide_spinner()?;
        self.process_pending_events();
        Ok(())
    }

    fn process_notification(
        &mut self,
        notification: Notification,
    ) -> AppResult<Option<Notification>> {
        match notification {
            Notification::Questions(NotifContent { content: tags, .. }) => {
                self.questions.items = vec![];
                if let Some(tag) = tags.into_iter().next() {
                    // if any topic change notification is received set jump to state to 0
                    if tag.id == "all" {
                        self.jump_to = 0;
                    };
                    let values = self.topic_tag_question_map[&tag].clone();
                    let notif = Notification::Stats(NotifContent::new(
                        WidgetName::QuestionList,
                        WidgetName::Stats,
                        values.to_vec(),
                    ));
                    self.questions.items = values;
                    self.selected_topic = Some(tag);
                    return Ok(Some(notif));
                }
            }
            Notification::SelectedItem(NotifContent { content, .. }) => {
                let (lookup_key, index) = content;
                match self.task_map.remove(&lookup_key).unwrap() {
                    (question, TaskType::Edit) => {
                        let question_id = question.borrow().frontend_question_id.clone();
                        let cached_question = self.cache.get(&question_id).unwrap();
                        let editor_data = cached_question
                            .editor_data
                            .as_ref()
                            .expect("no editor data found");
                        let question_data =
                            cached_question.qd.as_ref().expect("no question data found");
                        let description = question_data.html_to_text();
                        let slug = question_data.title_slug.as_str().to_string();

                        let snippets = &editor_data.code_snippets;
                        let selected_snippet = snippets[index].code.as_str().to_string();
                        let selected_lang = snippets[index].lang_slug.clone();
                        let dir = self.config.solutions_dir.clone();

                        let sf = SolutionFile::new(
                            slug,
                            selected_lang,
                            Some(description),
                            Some(selected_snippet),
                            question_id,
                        );
                        let save_path = sf.get_save_path(&dir);
                        sf.create_if_not_exists(&dir)?;
                        self.files
                            .entry(sf.question_id.parse().unwrap())
                            .or_default()
                            .insert(sf);

                        if let Err(e) = self.open_editor(&save_path) {
                            return Ok(Some(self.popup_paragraph_notification(
                                e.to_string(),
                                "Error opening editor".to_string(),
                                IndexSet::new(),
                            )));
                        };
                    }
                    (question, tt) => {
                        self.solution_file_popup_action(question, tt, index)?;
                    }
                }
            }

            Notification::Event(NotifContent {
                src_wid: _,
                dest_wid: _,
                content: event,
            }) => {
                return self.handler(event);
            }
            _ => {}
        }
        Ok(None)
    }

    fn get_common_state(&self) -> &CommonState {
        &self.common_state
    }

    fn get_common_state_mut(&mut self) -> &mut CommonState {
        &mut self.common_state
    }
    fn get_notification_queue(&mut self) -> &mut std::collections::VecDeque<Notification> {
        &mut self.common_state.notification_queue
    }
}
