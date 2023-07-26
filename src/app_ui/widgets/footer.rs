use crate::app_ui::channel::ChannelRequestSender;

use crate::errors::AppResult;

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

    fn process_notification(
        &mut self,
        notification: Notification,
    ) -> AppResult<Option<Notification>> {
        if let Notification::HelpText(NotifContent {
            src_wid: _,
            dest_wid: _,
            content,
        }) = notification
        {
            *self.get_help_texts_mut() = content;
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
