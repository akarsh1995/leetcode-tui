use std::time::{Duration, Instant};

use crate::app_ui::async_task_channel::ChannelRequestSender;

use crate::app_ui::components::color::TokyoNightColors;
use crate::errors::AppResult;

use ratatui::widgets::block::Position;
use ratatui::{prelude::*, widgets::Block};

use super::notification::{NotifContent, Notification, WidgetName};
use super::{CommonState, CrosstermStderr};

// Loading animation characters
const LOADING_CHARS: [char; 8] = ['⣾', '⣽', '⣻', '⢿', '⡿', '⣟', '⣯', '⣷'];

#[derive(Debug)]
pub struct HelpBar {
    pub common_state: CommonState,
    loading_state: usize,
    show_loading: bool,
    instant: Instant,
}

impl HelpBar {
    pub fn new(widget_name: WidgetName, task_sender: ChannelRequestSender) -> Self {
        let mut cs = CommonState::new(widget_name, task_sender, vec![]);
        cs.is_navigable = false;
        Self {
            common_state: cs,
            loading_state: 0,
            show_loading: false,
            instant: Instant::now(),
        }
    }
}

impl super::Widget for HelpBar {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let mut spans = vec![];
        for (i, ht) in self.get_help_texts().iter().enumerate() {
            let help_string: String = ht.into();
            let col: style::Color = TokyoNightColors::Foreground.into();
            spans.push(
                Span::from(help_string)
                    .fg(col)
                    .bg(TokyoNightColors::Selection.into()),
            );
            if i < self.get_help_texts().len() - 1 {
                spans.push(Span::from(" "))
            }
        }

        if self.show_loading {
            let elapsed = std::time::Instant::now() - self.instant;

            if elapsed > Duration::from_millis(80) {
                self.loading_state = (self.loading_state + 1) % LOADING_CHARS.len();
                self.instant = std::time::Instant::now();
            }

            frame.render_widget(
                Block::default()
                    .title(vec![Span::from(
                        LOADING_CHARS[self.loading_state].to_string(),
                    )])
                    .title_position(Position::Top)
                    .title_alignment(Alignment::Right),
                rect,
            );
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
        match notification {
            Notification::HelpText(NotifContent {
                src_wid: _,
                dest_wid: _,
                content,
            }) => {
                *self.get_help_texts_mut() = content;
            }
            Notification::Loading(NotifContent { content, .. }) => self.show_loading = content,
            _ => (),
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
