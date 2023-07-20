use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::app_ui::channel::Response;
use crate::app_ui::{channel::ChannelRequestSender, components::list::StatefulList};
use crate::entities::{QuestionModel, TopicTagModel};
use crate::errors::AppResult;

use crossterm::event::KeyEvent;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use super::notification::{Notification, NotificationRequestSender};
use super::{Callout, CrosstermStderr, StateManager, CHECK_MARK};

#[derive(Debug)]
pub struct QuestionListWidget {
    pub id: i32,
    pub task_sender: ChannelRequestSender,
    pub notification_sender: NotificationRequestSender,
    pub questions: StatefulList<QuestionModel>,
    pub all_questions: HashMap<Rc<TopicTagModel>, Vec<Rc<QuestionModel>>>,
    pub active: bool,
}

impl QuestionListWidget {
    pub fn new(
        id: i32,
        task_sender: ChannelRequestSender,
        notif_req_sender: NotificationRequestSender,
    ) -> Self {
        Self {
            id,
            task_sender,
            notification_sender: notif_req_sender,
            all_questions: HashMap::new(),
            active: false,
            questions: Default::default(),
        }
    }
}

impl StateManager for QuestionListWidget {
    fn set_active(&mut self) {
        self.active = true;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_inactive(&mut self) {
        self.active = false;
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

        let combination: Style;
        let qs_diff = question
            .difficulty
            .as_ref()
            .unwrap_or(&"Disabled".to_string())
            .to_string();

        combination = match qs_diff.as_str() {
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
}

impl super::Widget for QuestionListWidget {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let lines = self
            .questions
            .items
            .iter()
            .map(|q| Self::get_item(q))
            .collect::<Vec<_>>();

        let mut border_style = Style::default();
        if self.active {
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

    fn handler(&mut self, event: KeyEvent) -> AppResult<()> {
        match event.code {
            crossterm::event::KeyCode::Up => self.questions.previous(),
            crossterm::event::KeyCode::Down => self.questions.next(),
            _ => {}
        };
        Ok(())
    }

    fn setup(&mut self) -> AppResult<()> {
        self.task_sender
            .send(crate::app_ui::channel::TaskRequest::GetAllQuestionsMap { sender_id: self.id })?;
        Ok(())
    }

    fn process_task_response(&mut self, response: crate::app_ui::channel::TaskResponse) {
        match response {
            crate::app_ui::channel::TaskResponse::GetAllQuestionsMap(Response {
                content,
                sender_id: _,
            }) => {
                let map_iter = content.into_iter().map(|v| {
                    (
                        Rc::new(v.0),
                        (v.1.into_iter().map(|q| Rc::new(q))).collect::<Vec<_>>(),
                    )
                });
                self.all_questions.extend(map_iter)
            }
            _ => {}
        }
    }

    fn process_notification(&mut self, notification: &Notification) {
        match notification {
            Notification::UpdateQuestions(tags) => {
                self.questions.items = vec![];
                for tag in tags {
                    if tag.name.as_ref().map_or("all", |v| v) == "all" {
                        let mut question_set = HashSet::new();
                        for val in self.all_questions.values().flatten() {
                            question_set.insert(val.clone());
                        }
                        self.questions.items.extend(question_set.into_iter());
                    } else {
                        let values = self.all_questions.get(tag).unwrap();
                        for val in values {
                            self.questions.items.push(val.clone());
                        }
                    };
                }
                self.questions.items.sort();
            }
            _ => {}
        }
    }

    fn set_response(&mut self) {}
}
