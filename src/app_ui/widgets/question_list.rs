use std::collections::{HashMap, HashSet, VecDeque};
use std::process::Stdio;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::app_ui::channel::{Request, Response, TaskResponse};
use crate::app_ui::components::help_text::{CommonHelpText, HelpText};
use crate::app_ui::components::popups::paragraph::ParagraphPopup;
use crate::app_ui::event::VimPingSender;
use crate::app_ui::{channel::ChannelRequestSender, components::list::StatefulList};
use crate::entities::{QuestionModel, TopicTagModel};
use crate::errors::AppResult;

use crossterm::event::{KeyCode, KeyEvent};
use indexmap::IndexSet;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use super::notification::{NotifContent, Notification, PopupMessage, PopupType, WidgetName};
use super::{Callout, CommonState, CrosstermStderr, Widget, CHECK_MARK};
use lru;
use std::num::NonZeroUsize;

#[derive(Debug, Default)]
struct CachedQuestion {
    qd: Option<TaskResponse>,
}

#[derive(Debug)]
pub struct QuestionListWidget {
    pub common_state: CommonState,
    pub questions: StatefulList<QuestionModel>,
    pub all_questions: HashMap<Rc<TopicTagModel>, Vec<Rc<QuestionModel>>>,
    popup_events: IndexSet<HelpText>,
    vim_tx: VimPingSender,
    vim_running: Arc<AtomicBool>,
    cache: lru::LruCache<Rc<QuestionModel>, CachedQuestion>,
    task_map: HashMap<String, Rc<QuestionModel>>,
    pending_event_actions: VecDeque<(KeyEvent, Rc<QuestionModel>)>,
}

impl QuestionListWidget {
    pub fn new(
        id: WidgetName,
        task_sender: ChannelRequestSender,
        vim_tx: VimPingSender,
        vim_running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            popup_events: IndexSet::from_iter([
                // send the events that this widget can handle
                CommonHelpText::Solve.into(),
                CommonHelpText::Run.into(),
                CommonHelpText::Submit.into(),
            ]),
            common_state: CommonState::new(
                id,
                task_sender,
                vec![
                    CommonHelpText::SwitchPane.into(),
                    CommonHelpText::ScrollUp.into(),
                    CommonHelpText::ScrollDown.into(),
                    CommonHelpText::Solve.into(),
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
        }
    }
}

impl QuestionListWidget {
    fn get_item(question: &QuestionModel) -> ListItem {
        let number = question.frontend_question_id.clone();
        let title = question
            .title
            .as_ref()
            .unwrap_or(&"No title".to_string())
            .to_string();

        let is_accepted = question
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

    fn process_pending_events(&mut self) {
        while let Some((remaining_event, qm)) = self.pending_event_actions.pop_front() {
            match remaining_event.code {
                KeyCode::Enter => {
                    let ques_in_cache = self
                        .cache
                        .get_or_insert_mut(qm.clone(), CachedQuestion::default);
                    if let Some(cache_ques) = &ques_in_cache.qd {
                        let popup_content = match cache_ques {
                            TaskResponse::QuestionDetail(qd) => qd.content.html_to_text(),
                            TaskResponse::Error(e) => e.content.clone(),
                            _ => break,
                        };
                        let title = qm.title.as_ref().unwrap();
                        self.common_state
                            .notification_queue
                            .push_back(Notification::Popup(NotifContent {
                                src_wid: self.get_widget_name(),
                                dest_wid: WidgetName::Popup,
                                content: PopupMessage {
                                    help_texts: self.popup_events.clone(),
                                    popup: PopupType::Paragraph(ParagraphPopup::new(
                                        title.to_string(),
                                        popup_content,
                                    )),
                                },
                            }));
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
}

impl super::Widget for QuestionListWidget {
    fn set_active(&mut self) -> AppResult<Option<Notification>> {
        self.get_common_state_mut().active = true;
        Ok(Some(Notification::HelpText(NotifContent::new(
            WidgetName::QuestionList,
            WidgetName::HelpLine,
            self.get_help_texts().clone(),
        ))))
    }

    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let lines = self
            .questions
            .items
            .iter()
            .map(|q| Self::get_item(q))
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
                let selected_question = self.questions.get_selected_item();
                if let Some(sel) = selected_question {
                    let model = sel.clone();

                    let cached_q = self
                        .cache
                        .get_or_insert_mut(model.clone(), CachedQuestion::default);

                    // if question details are not in the cache then send request to get the
                    // details and push the event to be processed in the future
                    if cached_q.qd.as_ref().is_none() {
                        if let Some(title_slug) = model.title_slug.as_ref() {
                            // before sending the task request set the key as the request id and value
                            // as question model so that we can obtain the question model once we get the response
                            self.task_map.insert(title_slug.to_string(), model.clone());
                            self.get_task_sender().send(
                                crate::app_ui::channel::TaskRequest::QuestionDetail(Request {
                                    widget_name: self.get_widget_name(),
                                    request_id: title_slug.clone(),
                                    content: title_slug.clone(),
                                }),
                            )?;
                            self.pending_event_actions.push_back((event, model.clone()));
                        };
                    } else {
                        // because the question details exists in the cache
                        // process instantly send the notification to notification queue send the
                        // notification to notification queue
                        self.pending_event_actions.push_back((event, model.clone()));
                        self.process_pending_events();
                    }
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                let vim_cmd = "nvim".to_string();
                let mut output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&vim_cmd)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("Can run vim cmd");
                self.vim_running
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                let vim_cmd_result = output.wait().expect("Run exits ok");
                self.vim_tx.blocking_send(1).unwrap();
                self.vim_running
                    .store(false, std::sync::atomic::Ordering::Relaxed);
                if !vim_cmd_result.success() {
                    println!("error vim");
                }
            }
            _ => {}
        };
        Ok(None)
    }

    fn setup(&mut self) -> AppResult<Option<Notification>> {
        self.get_task_sender()
            .send(crate::app_ui::channel::TaskRequest::GetAllQuestionsMap(
                Request {
                    widget_name: self.get_widget_name(),
                    request_id: "".to_string(),
                    content: (),
                },
            ))?;
        Ok(None)
    }

    fn process_task_response(
        &mut self,
        response: crate::app_ui::channel::TaskResponse,
    ) -> AppResult<Option<Notification>> {
        match &response {
            crate::app_ui::channel::TaskResponse::GetAllQuestionsMap(Response {
                content, ..
            }) => {
                // avoid this cloning
                let map_iter = content.iter().map(|v| {
                    (
                        Rc::new(v.0.clone()),
                        (v.1.iter().map(|x| Rc::new(x.clone()))).collect::<Vec<_>>(),
                    )
                });
                self.all_questions.extend(map_iter);
                for ql in &mut self.all_questions.values_mut() {
                    ql.sort_unstable()
                }
                return Ok(Some(Notification::Questions(NotifContent::new(
                    WidgetName::QuestionList,
                    super::notification::WidgetName::QuestionList,
                    vec![TopicTagModel {
                        name: Some("All".to_owned()),
                        id: "all".to_owned(),
                        slug: Some("all".to_owned()),
                    }],
                ))));
            }
            crate::app_ui::channel::TaskResponse::QuestionDetail(qd) => {
                let cached_q = self.cache.get_or_insert_mut(
                    self.task_map
                        .remove(&qd.request_id)
                        .expect("sent task is not found in the task list."),
                    CachedQuestion::default,
                );
                cached_q.qd = Some(response);
                self.process_pending_events();
            }
            _ => {}
        }
        Ok(None)
    }

    fn process_notification(
        &mut self,
        notification: Notification,
    ) -> AppResult<Option<Notification>> {
        match notification {
            Notification::Questions(NotifContent {
                src_wid: _,
                dest_wid: _,
                content: tags,
            }) => {
                self.questions.items = vec![];
                if let Some(tag) = tags.into_iter().next() {
                    if tag.id == "all" {
                        let mut question_set = HashSet::new();
                        for val in self.all_questions.values().flatten() {
                            question_set.insert(val.clone());
                        }
                        let notif = Notification::Stats(NotifContent::new(
                            WidgetName::QuestionList,
                            WidgetName::Stats,
                            question_set
                                .clone()
                                .into_iter()
                                .map(|q| q.as_ref().clone())
                                .collect::<Vec<_>>(),
                        ));
                        self.questions.items.extend(question_set.into_iter());
                        self.questions.items.sort_unstable();
                        return Ok(Some(notif));
                    } else {
                        let values = self.all_questions.get(&tag).unwrap();
                        let notif = Notification::Stats(NotifContent::new(
                            WidgetName::QuestionList,
                            WidgetName::Stats,
                            values
                                .iter()
                                .map(|x| x.as_ref().clone())
                                .collect::<Vec<_>>(),
                        ));
                        self.questions.items.extend(values.iter().cloned());
                        return Ok(Some(notif));
                    };
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

// send popup for showing the question details.
// if let Some(sel) = selected_question {
// }
