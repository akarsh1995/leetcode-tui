use crate::app_ui::helpers::question::QuestionModelContainer;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::app_ui::async_task_channel::{Request, Response, TaskResponse};
use crate::app_ui::components::help_text::{CommonHelpText, HelpText};
use crate::app_ui::components::popups::paragraph::ParagraphPopup;
use crate::app_ui::components::popups::selection_list::SelectionListPopup;
use crate::app_ui::event::VimPingSender;
use crate::app_ui::helpers::utils::{generate_random_string, SolutionFile};
use crate::app_ui::{async_task_channel::ChannelRequestSender, components::list::StatefulList};
use crate::config::Config;
use crate::deserializers;
use crate::deserializers::editor_data::CodeSnippet;
use crate::deserializers::run_submit::{ParsedResponse, Success};
use crate::entities::TopicTagModel;
use crate::errors::AppResult;
use crate::graphql::run_code::RunCode;
use crate::graphql::submit_code::SubmitCode;
use crate::graphql::{Language, RunOrSubmitCode};

use crossterm::event::{KeyCode, KeyEvent};
use indexmap::IndexSet;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use super::notification::{NotifContent, Notification, PopupMessage, PopupType, WidgetName};
use super::{CommonState, CrosstermStderr, Widget};
use crate::app_ui::components::color::{Callout, CHECK_MARK};
use lru;
use std::num::NonZeroUsize;

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
enum TaskType {
    Run,
    Edit,
    Read,
    Submit,
}

type Question = Rc<QuestionModelContainer>;

#[derive(Debug)]
pub struct QuestionListWidget {
    pub common_state: CommonState,
    pub questions: StatefulList<QuestionModelContainer>,
    pub all_questions: HashMap<Rc<TopicTagModel>, Vec<Question>>,
    vim_tx: VimPingSender,
    vim_running: Arc<AtomicBool>,
    cache: lru::LruCache<Question, CachedQuestion>,
    task_map: HashMap<String, (Question, TaskType)>,
    pending_event_actions: IndexSet<(KeyEvent, Question)>,
    config: Rc<Config>,
    files: HashMap<i32, HashSet<SolutionFile>>,
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
            .questions_dir
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
            all_questions: HashMap::new(),
            questions: Default::default(),
            vim_tx,
            vim_running,
            cache: lru::LruCache::new(NonZeroUsize::new(10).unwrap()),
            task_map: HashMap::new(),
            pending_event_actions: Default::default(),
            config,
            files,
        }
    }
}

impl QuestionListWidget {
    fn send_fetch_question_editor_details(&mut self, question: Question) -> AppResult<()> {
        if let Some(cached_q) = self.cache.peek(&question) {
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
                    content: question
                        .question
                        .borrow()
                        .title_slug
                        .as_ref()
                        .unwrap()
                        .clone(),
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
                question_id: question.question.borrow().frontend_question_id.clone(),
                typed_code,
                slug: question
                    .question
                    .borrow()
                    .title_slug
                    .as_ref()
                    .unwrap()
                    .clone(),
            };

            RunOrSubmitCode::Submit(submit_code)
        } else {
            let run_code = RunCode {
                lang,
                question_id: question.question.borrow().frontend_question_id.clone(),
                typed_code,
                test_cases_stdin: None, // automatically fetches sample test cases from the server
                slug: question
                    .question
                    .borrow()
                    .title_slug
                    .as_ref()
                    .unwrap()
                    .clone(),
            };

            RunOrSubmitCode::Run(run_code)
        };

        self.get_task_sender()
            .send(
                crate::app_ui::async_task_channel::TaskRequest::CodeRunRequest(Request {
                    widget_name: self.get_widget_name(),
                    request_id: random_key.clone(),
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
                    .question
                    .borrow()
                    .frontend_question_id
                    .clone()
                    .parse()
                    .unwrap(),
            )
            .expect("Question id does not exist in the solutions mapping");
        let solution_file = solution_files.iter().nth(index).unwrap();
        let typed_code = solution_file.read_file_contents(&self.config.questions_dir);
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
                    content: question
                        .question
                        .borrow()
                        .title_slug
                        .as_ref()
                        .unwrap()
                        .clone(),
                }),
            )
            .map_err(Box::new)?;
        Ok(())
    }

    fn sync_db_solution_submit_status(&mut self, question: Question) -> AppResult<()> {
        self.show_spinner()?;
        self.get_task_sender()
            .send(
                crate::app_ui::async_task_channel::TaskRequest::DbUpdateQuestion(Request {
                    widget_name: self.get_widget_name(),
                    request_id: "".to_string(),
                    content: question.question.borrow().to_owned(),
                }),
            )
            .map_err(Box::new)?;
        Ok(())
    }

    fn is_notif_pending(&self, key: &(KeyEvent, Question)) -> bool {
        self.pending_event_actions.contains(key)
    }

    fn open_vim_editor(&mut self, file_name: &Path) {
        let vim_cmd = format!("nvim {}", file_name.display());
        let mut output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&vim_cmd)
            .spawn()
            .expect("Can't run vim cmd");
        self.vim_running
            .store(true, std::sync::atomic::Ordering::Relaxed);
        let vim_cmd_result = output.wait().expect("Run exits ok");
        self.vim_running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.vim_tx.blocking_send(1).unwrap();
        if !vim_cmd_result.success() {
            println!("error vim");
        }
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

    fn get_item(question: &Rc<QuestionModelContainer>) -> ListItem {
        let number = question.question.borrow().frontend_question_id.clone();
        let title = question
            .question
            .borrow()
            .title
            .as_ref()
            .unwrap_or(&"No title".to_string())
            .to_string();

        let is_accepted = question
            .question
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

        let qs_diff = question
            .question
            .borrow()
            .difficulty
            .as_ref()
            .unwrap_or(&"Disabled".to_string())
            .to_string();

        let combination: Style = match qs_diff.as_str() {
            "Easy" => Callout::Success.get_pair().fg,
            "Medium" => Callout::Warning.get_pair().fg,
            "Hard" => Callout::Error.get_pair().fg,
            "Disabled" => Callout::Disabled.get_pair().fg,
            _ => unimplemented!(),
        }
        .into();

        let styled_title = Span::styled(line_text, combination);
        ListItem::new(styled_title)
    }

    fn add_event_to_event_queue(&mut self, data: (KeyEvent, Question)) -> bool {
        self.pending_event_actions.insert(data)
    }

    fn process_pending_events(&mut self) {
        let mut to_process_again = vec![];
        while let Some((pending_event, qm)) = self.pending_event_actions.pop() {
            let ques_in_cache = self
                .cache
                .get_or_insert_mut(qm.clone(), CachedQuestion::default);
            match pending_event.code {
                KeyCode::Enter => {
                    if let Some(cache_ques) = &ques_in_cache.qd {
                        let content = cache_ques.html_to_text();
                        let title = qm.question.borrow().title.as_ref().unwrap().to_string();
                        let notif = self.popup_paragraph_notification(
                            content,
                            title,
                            IndexSet::from_iter([CommonHelpText::Edit.into()]),
                        );
                        self.get_notification_queue().push_back(notif);
                    } else {
                        to_process_again.push((pending_event, qm));
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
                            .insert(popup_key.clone(), (qm.clone(), TaskType::Edit));
                        let notif = self.popup_list_notification(
                            content,
                            title,
                            popup_key,
                            IndexSet::new(),
                        );
                        self.get_notification_queue().push_back(notif);
                    } else {
                        to_process_again.push((pending_event, qm));
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
        let k = self
            .cache
            .get_or_insert_mut(model.clone(), CachedQuestion::default);
        (k, model.clone())
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
            .question
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
        Ok(None)
    }
}

impl super::Widget for QuestionListWidget {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let lines = self
            .questions
            .items
            .iter()
            .map(Self::get_item)
            .collect::<Vec<_>>();

        let mut border_style = Style::default();
        if self.is_active() {
            border_style = border_style.fg(Color::Cyan);
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
                    .bg(Color::Rgb(0, 0, 0))
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_stateful_widget(items, rect, &mut self.questions.state);
    }

    fn handler(&mut self, event: KeyEvent) -> AppResult<Option<Notification>> {
        match event.code {
            crossterm::event::KeyCode::Up => self.questions.previous(),
            crossterm::event::KeyCode::Down => self.questions.next(),
            crossterm::event::KeyCode::Enter => {
                let (cache, model) = self.get_selected_question_from_cache();
                let question_data_in_cache = cache.question_data_received();

                if question_data_in_cache {
                    let content = cache.get_question_content().unwrap();
                    let title = model.question.borrow().title.as_ref().unwrap().clone();
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
                self.add_event_to_event_queue((event, model));
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
                        .insert(popup_key.clone(), (model.clone(), TaskType::Edit));
                    let notif =
                        self.popup_list_notification(content, title, popup_key, IndexSet::new());
                    return Ok(Some(notif));
                }

                if self.is_notif_pending(&(event, model.clone())) {
                    self.process_pending_events();
                    return Ok(None);
                }

                // before sending the task request set the key as the request id and value
                // as question model so that we can obtain the question model once we get the response
                self.send_fetch_question_editor_details(model.clone())?;
                self.add_event_to_event_queue((event, model));
            }

            KeyCode::Char('r') | KeyCode::Char('R') => {
                return self.run_or_submit_code_event_handler(TaskType::Run);
            }
            KeyCode::Char('s') => {
                return self.run_or_submit_code_event_handler(TaskType::Submit);
            }
            _ => {}
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

    fn process_task_response(
        &mut self,
        response: crate::app_ui::async_task_channel::TaskResponse,
    ) -> AppResult<()> {
        match response {
            crate::app_ui::async_task_channel::TaskResponse::GetAllQuestionsMap(Response {
                content,
                ..
            }) => {
                // creating rc cloned question as one question can appear in multiple topics
                let question_set = content
                    .iter()
                    .flat_map(|x| {
                        x.1.iter().map(|x| {
                            (
                                x.frontend_question_id.clone(),
                                Rc::new(QuestionModelContainer {
                                    question: RefCell::new(x.clone()),
                                }),
                            )
                        })
                    })
                    .collect::<HashMap<_, _>>();

                let map_iter = content.into_iter().map(|v| {
                    (
                        Rc::new(v.0),
                        (v.1.into_iter()
                            .map(|x| question_set.get(&x.frontend_question_id).unwrap().clone()))
                        .collect::<Vec<_>>(),
                    )
                });

                self.all_questions.extend(map_iter);
                for ql in &mut self.all_questions.values_mut() {
                    ql.sort_unstable()
                }
                self.get_notification_queue()
                    .push_back(Notification::Questions(NotifContent::new(
                        WidgetName::QuestionList,
                        super::notification::WidgetName::QuestionList,
                        vec![TopicTagModel {
                            name: Some("All".to_owned()),
                            id: "all".to_owned(),
                            slug: Some("all".to_owned()),
                        }],
                    )));
            }
            crate::app_ui::async_task_channel::TaskResponse::QuestionDetail(qd) => {
                let cached_q = self.cache.get_or_insert_mut(
                    self.task_map
                        .remove(&qd.request_id)
                        .expect("sent task is not found in the task list.")
                        .0,
                    CachedQuestion::default,
                );
                cached_q.qd = Some(qd.content);
            }
            TaskResponse::QuestionEditorData(ed) => {
                let cached_q = self.cache.get_or_insert_mut(
                    self.task_map
                        .remove(&ed.request_id)
                        .expect("sent task is not found in the task list.")
                        .0,
                    CachedQuestion::default,
                );
                cached_q.editor_data = Some(ed.content);
            }
            TaskResponse::RunResponseData(run_res) => {
                let mut is_submit = false;
                let k = match run_res.content {
                    ParsedResponse::Pending => "Pending".to_string(),
                    ParsedResponse::CompileError(_) => "Compile Error".to_string(),
                    ParsedResponse::RuntimeError(_) => {
                        // } => format!("{status_msg}:\n\n{runtime_error}\n\n{full_runtime_error}"),
                        "Runtime Error".to_string()
                    }
                    ParsedResponse::MemoryLimitExceeded(_) => "Memory Limit Exceeded".to_string(),
                    ParsedResponse::OutputLimitExceed(_) => "Output Limit Exceeded".to_string(),
                    ParsedResponse::TimeLimitExceeded(_) => "Time Limit Exceeded".to_string(),
                    ParsedResponse::InternalError(_) => "Internal Error".to_string(),
                    ParsedResponse::TimeOut(_) => "Timout".to_string(),
                    ParsedResponse::Success(Success::Run {
                        status_runtime,
                        code_answer,
                        expected_code_answer,
                        correct_answer,
                        total_correct,
                        total_testcases,
                        status_memory,
                        ..
                    }) => {
                        let is_accepted_symbol = if correct_answer { "✅" } else { "❌" };
                        let mut ans_compare = String::new();
                        for (output, expected_output) in
                            code_answer.into_iter().zip(expected_code_answer)
                        {
                            let emoji = if output == expected_output {
                                "✅"
                            } else {
                                "❌"
                            };
                            let compare = format!(
                                "{emoji}\nOuput: {}\nExpected: {}\n\n",
                                output, expected_output
                            );
                            ans_compare.push_str(compare.as_str())
                        }
                        let result_string = vec![
                            format!("Accepted: {}", is_accepted_symbol),
                            if let Some(correct) = total_correct {
                                let mut x = format!("Correct: {correct}");
                                if let Some(total) = total_testcases {
                                    x = format!("{x}/{}", total);
                                }
                                x
                            } else {
                                String::new()
                            },
                            format!("Memory Used: {status_memory}"),
                            format!("Status Runtime: {status_runtime}"),
                            ans_compare,
                        ];
                        result_string.join("\n")
                    }
                    ParsedResponse::Success(Success::Submit {
                        status_runtime,
                        total_correct,
                        total_testcases,
                        status_memory,
                        ..
                    }) => {
                        // upon successful submit of the question update the question accepted status
                        // also update the db
                        {
                            let question_model_container = self
                                .task_map
                                .get(&run_res.request_id)
                                .expect(
                                "Cannot get the question model container from the sent task map.",
                            );
                            question_model_container.0.question.borrow_mut().status =
                                Some("ac".to_string());
                            self.sync_db_solution_submit_status(
                                question_model_container.0.clone(),
                            )?;
                        }
                        is_submit = true;
                        let is_accepted_symbol = "✅";
                        let result_string = vec![
                            format!("Accepted: {}", is_accepted_symbol),
                            if let Some(correct) = total_correct {
                                let mut x = format!("Correct: {correct}");
                                if let Some(total) = total_testcases {
                                    x = format!("{x}/{}", total);
                                }
                                x
                            } else {
                                String::new()
                            },
                            format!("Memory Used: {status_memory}"),
                            format!("Status Runtime: {status_runtime}"),
                        ];
                        result_string.join("\n")
                    }
                    ParsedResponse::Unknown(_) => "Unknown Error".to_string(),
                };
                let notification = self.popup_paragraph_notification(
                    k.to_string(),
                    format!("{} Status", (if is_submit { "Submit" } else { "Run" })),
                    IndexSet::new(),
                );
                // post submit remove the reference_task_key from task_map
                self.task_map.remove(&run_res.request_id).unwrap();
                self.get_notification_queue().push_back(notification);
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
                    if tag.id == "all" {
                        let mut unique_question_map = HashMap::new();
                        for val in self.all_questions.values().flatten() {
                            unique_question_map.insert(
                                val.question.borrow().frontend_question_id.clone(),
                                val.clone(),
                            );
                        }
                        let unique_questions = unique_question_map
                            .drain()
                            .map(|(_, v)| v)
                            .collect::<Vec<_>>();
                        let notif = Notification::Stats(NotifContent::new(
                            WidgetName::QuestionList,
                            WidgetName::Stats,
                            unique_questions.clone(),
                        ));
                        self.questions.items.extend(unique_questions);
                        self.questions.items.sort_unstable();
                        return Ok(Some(notif));
                    } else {
                        let values = self.all_questions.get(&tag).unwrap();
                        let notif = Notification::Stats(NotifContent::new(
                            WidgetName::QuestionList,
                            WidgetName::Stats,
                            values.to_vec(),
                        ));
                        self.questions.items.extend(values.iter().cloned());
                        return Ok(Some(notif));
                    };
                }
            }
            Notification::SelectedItem(NotifContent { content, .. }) => {
                let (lookup_key, index) = content;
                match self.task_map.remove(&lookup_key).unwrap() {
                    (question, TaskType::Edit) => {
                        let question_id = question.question.borrow().frontend_question_id.clone();
                        let cached_question = self.cache.get(&question).unwrap();
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
                        let dir = self.config.questions_dir.clone();

                        let sf = SolutionFile::new(
                            slug,
                            selected_lang,
                            Some(description),
                            Some(selected_snippet),
                            question_id.to_string(),
                        );
                        sf.create_if_not_exists(&dir)?;
                        self.open_vim_editor(&sf.get_save_path(&dir));
                        self.files
                            .entry(sf.question_id.parse().unwrap())
                            .or_default()
                            .insert(sf);
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
