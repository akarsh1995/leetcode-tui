use crate::{
    app_ui::{
        channel::{ChannelRequestSender, TaskResponse},
        components::rect::centered_rect,
    },
    errors::AppResult,
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use super::{
    notification::{Notification, NotificationRequestSender},
    CrosstermStderr, StateManager, Widget,
};

#[derive(Debug)]
pub struct Popup {
    pub id: i32,
    pub task_sender: ChannelRequestSender,
    pub notification_sender: NotificationRequestSender,
    pub message: String,
    pub title: String,
    pub active: bool,
    pub scroll_x: u16,
    pub scroll_y: u16,
}

impl Popup {
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
            message: "No message so far".to_string(),
            title: "Popup".to_string(),
            scroll_x: 0,
            scroll_y: 0,
        }
    }
}

impl Popup {
    fn create_block(&self) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                self.title.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ))
    }
}

impl StateManager for Popup {
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

impl Widget for Popup {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        if self.active {
            let size = rect;

            let size = centered_rect(60, 50, size);

            let block = self.create_block();
            let content = Paragraph::new(self.message.to_owned())
                .wrap(Wrap { trim: true })
                .scroll((self.scroll_y, self.scroll_x))
                .block(block);

            frame.render_widget(Clear, size);
            frame.render_widget(content, size); // frame.render_widget(block, area);
        }
    }

    fn handler(&mut self, event: KeyEvent) -> AppResult<()> {
        match event.code {
            crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Esc => {
                self.active = false
            }
            KeyCode::Up => self.scroll_y = self.scroll_y.saturating_sub(1),
            KeyCode::Down => self.scroll_y += 1,
            _ => (),
        }
        Ok(())
    }

    fn process_task_response(&mut self, _response: TaskResponse) -> AppResult<()> {
        Ok(())
    }

    fn setup(&mut self) -> AppResult<()> {
        Ok(())
    }

    fn set_response(&mut self) {}

    fn process_notification(&mut self, notification: &Notification) -> AppResult<()> {
        if let Notification::Popup(pop_msg) = notification {
            self.message = pop_msg.message.to_owned();
            self.title = pop_msg.title.to_owned();
            self.active = true;
        }
        Ok(())
    }
}
