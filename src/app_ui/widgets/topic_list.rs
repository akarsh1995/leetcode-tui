use crate::{
    app_ui::{
        channel::{ChannelRequestSender, Response, TaskRequest, TaskResponse},
        components::list::StatefulList,
    },
    entities::TopicTagModel,
    errors::{AppResult, LcAppError},
};

use crossterm::event::KeyEvent;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use super::{
    notification::{Notification, NotificationRequestSender},
    Callout, CrosstermStderr, StateManager, Widget,
};

#[derive(Debug)]
pub struct TopicTagListWidget {
    pub id: i32,
    pub task_sender: ChannelRequestSender,
    pub notification_sender: NotificationRequestSender,
    pub topics: StatefulList<TopicTagModel>,
    pub active: bool,
}

impl TopicTagListWidget {
    pub fn new(
        id: i32,
        task_sender: ChannelRequestSender,
        notif_req_sender: NotificationRequestSender,
    ) -> Self {
        Self {
            id,
            task_sender,
            notification_sender: notif_req_sender,
            active: false,
            topics: Default::default(),
        }
    }
}

impl TopicTagListWidget {
    fn get_item(ttm: &TopicTagModel) -> ListItem {
        ListItem::new(Text::styled(
            ttm.name
                .as_ref()
                .map_or("Not a Valid Tag".to_string(), |name| name.to_owned()),
            Style::default(),
        ))
    }

    fn update_questions(&mut self) -> AppResult<()> {
        if let Some(sel) = self.topics.get_selected_item() {
            let questions = vec![sel.as_ref().clone()];
            self.notification_sender
                .send(Notification::UpdateQuestions(questions))
                .map_err(|e| LcAppError::NotificationSendError(e))?;
        }
        Ok(())
    }
}

impl StateManager for TopicTagListWidget {
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

impl Widget for TopicTagListWidget {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let lines = self
            .topics
            .items
            .iter()
            .map(|tt| Self::get_item(tt))
            .collect::<Vec<_>>();

        let mut border_style = Style::default();

        if self.active {
            border_style = border_style.fg(Color::Cyan);
        }

        let hstyle: Style = Callout::Info.into();
        let items = List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Topics")
                    .border_style(border_style),
            )
            .highlight_style(hstyle.add_modifier(Modifier::BOLD));
        frame.render_stateful_widget(items, rect, &mut self.topics.state);
    }

    fn handler(&mut self, event: KeyEvent) -> AppResult<()> {
        match event.code {
            crossterm::event::KeyCode::Up => {
                self.topics.previous();
                self.update_questions()?;
            }
            crossterm::event::KeyCode::Down => {
                self.topics.next();
                self.update_questions()?;
            }
            _ => {}
        };
        Ok(())
    }

    fn process_task_response(&mut self, response: TaskResponse) {
        match response {
            TaskResponse::AllTopicTags(Response {
                content,
                sender_id: _,
            }) => {
                self.topics.add_item(TopicTagModel {
                    name: Some("All".to_owned()),
                    id: "all".to_owned(),
                    slug: Some("all".to_owned()),
                });
                for tt in content {
                    self.topics.add_item(tt)
                }
            }
            _ => {}
        }
    }

    fn setup(&mut self) -> AppResult<()> {
        self.task_sender
            .send(TaskRequest::GetAllTopicTags { sender_id: self.id })?;
        Ok(())
    }

    fn set_response(&mut self) {}

    fn process_notification(&mut self, _notification: &Notification) {}
}
