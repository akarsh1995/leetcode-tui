use crate::{
    app_ui::{
        channel::{ChannelRequestSender, TaskResponse},
        components::{
            help_text::{CommonHelpText, HelpText},
            rect::centered_rect,
        },
    },
    errors::AppResult,
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use super::{
    notification::{NotifContent, Notification, WidgetName},
    CommonState, CrosstermStderr, Widget,
};

#[derive(Debug)]
pub struct Popup {
    pub common_state: CommonState,
    pub message: String,
    pub title: String,
    pub scroll_x: u16,
    pub scroll_y: u16,
    pub callee_wid: Option<WidgetName>,
}

impl Popup {
    pub fn new(widget_name: WidgetName, task_sender: ChannelRequestSender) -> Self {
        Self {
            common_state: CommonState::new(
                widget_name,
                task_sender,
                vec![CommonHelpText::Close.into()],
            ),
            message: "No message so far".to_string(),
            title: "Popup".to_string(),
            scroll_x: 0,
            callee_wid: None,
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

impl Widget for Popup {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        if self.is_active() {
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

    fn handler(&mut self, event: KeyEvent) -> AppResult<Option<Notification>> {
        match event.code {
            crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Esc => {
                self.set_inactive()
            }
            KeyCode::Up => self.scroll_y = self.scroll_y.saturating_sub(1),
            KeyCode::Down => self.scroll_y += 1,
            _ => {
                // in case popup has helptext to take the event but does not have associated key in
                // the handler mapping pass it to the popup callee
                return Ok(Some(Notification::Event(NotifContent {
                    src_wid: self.get_widget_name(),
                    dest_wid: self.callee_wid.as_ref().unwrap().clone(),
                    content: event,
                })));
            }
        }
        Ok(None)
    }

    fn process_task_response(
        &mut self,
        _response: TaskResponse,
    ) -> AppResult<Option<Notification>> {
        Ok(None)
    }

    fn setup(&mut self) -> AppResult<Option<Notification>> {
        Ok(None)
    }

    fn process_notification(
        &mut self,
        notification: &Notification,
    ) -> AppResult<Option<Notification>> {
        if let Notification::Popup(NotifContent {
            src_wid,
            dest_wid: _,
            content,
        }) = notification
        {
            self.message = content.message.to_owned();
            self.title = content.title.to_owned();
            self.callee_wid = Some(src_wid.clone());
            self.get_help_texts_mut().extend(content.help_texts.clone());
            return Ok(Some(Notification::HelpText(NotifContent {
                src_wid: self.common_state.widget_name.clone(),
                dest_wid: WidgetName::HelpLine,
                content: self.get_help_texts().clone(),
            })));
        }
        Ok(None)
    }

    fn get_common_state(&self) -> &CommonState {
        &self.common_state
    }

    fn get_common_state_mut(&mut self) -> &mut CommonState {
        &mut self.common_state
    }
}
