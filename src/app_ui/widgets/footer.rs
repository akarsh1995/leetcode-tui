use std::collections::HashSet;

use crate::app_ui::channel::ChannelRequestSender;
use crate::app_ui::components::help_text::HelpText;
use crate::errors::AppResult;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::block::Position;
use ratatui::{prelude::*, widgets::Block};

use super::notification::{NotifContent, Notification, WidgetName};
use super::{CommonState, CrosstermStderr};

#[derive(Debug)]
pub struct Footer {
    pub common_state: CommonState,
}

impl Footer {
    pub fn new(widget_name: WidgetName, task_sender: ChannelRequestSender) -> Self {
        let mut cs = CommonState::new(widget_name, task_sender, vec![]);
        cs.is_navigable = false;
        Self { common_state: cs }
    }
}

impl super::Widget for Footer {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let mut spans = vec![];
        for (i, ht) in self.get_help_texts().iter().enumerate() {
            let help_string: String = ht.into();
            spans.push(Span::from(help_string).white().on_cyan());
            if i < self.get_help_texts().len() - 1 {
                spans.push(Span::from(" "))
            }
        }

        if !spans.is_empty() {
            let b = Block::default()
                .title(spans)
                .title_position(Position::Bottom)
                .title_alignment(Alignment::Right);

            frame.render_widget(b, rect);
        }
    }

    fn handler(&mut self, _event: KeyEvent) -> AppResult<Option<Notification>> {
        Ok(None)
    }

    fn setup(&mut self) -> AppResult<Option<Notification>> {
        Ok(None)
    }

    fn process_task_response(
        &mut self,
        _response: crate::app_ui::channel::TaskResponse,
    ) -> AppResult<Option<Notification>> {
        Ok(None)
    }

    fn process_notification(
        &mut self,
        notification: &Notification,
    ) -> AppResult<Option<Notification>> {
        if let Notification::HelpText(NotifContent {
            src_wid: _,
            dest_wid: _,
            content,
        }) = notification
        {
            *self.get_help_texts_mut() = content.clone();
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
