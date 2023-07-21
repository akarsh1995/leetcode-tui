use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::app_ui::channel::Response;
use crate::app_ui::components::help_text::HelpText;
use crate::app_ui::{channel::ChannelRequestSender, components::list::StatefulList};
use crate::entities::{QuestionModel, TopicTagModel};
use crate::errors::AppResult;

use crossterm::event::{KeyCode, KeyEvent, ModifierKeyCode};
use ratatui::widgets::block::{Position, Title};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use super::notification::{Notification, NotificationRequestSender, PopupMessage};
use super::{Callout, CommonState, CrosstermStderr, CHECK_MARK};

#[derive(Debug)]
pub struct Footer {
    pub common_state: CommonState,
    pub helptexts: Vec<HelpText>,
}

impl Footer {
    pub fn new(
        id: i32,
        task_sender: ChannelRequestSender,
        notif_req_sender: NotificationRequestSender,
    ) -> Self {
        let mut cs = CommonState::new(id, task_sender, notif_req_sender);
        cs.is_navigable = false;
        Self {
            common_state: cs,
            helptexts: vec![],
        }
    }
}

impl super::Widget for Footer {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let mut spans = vec![];
        for (i, ht) in self.helptexts.iter().enumerate() {
            let help_string: String = ht.into();
            spans.push(Span::from(help_string).white().on_cyan());
            if i < self.helptexts.len() - 1 {
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

    fn handler(&mut self, event: KeyEvent) -> AppResult<()> {
        Ok(())
    }

    fn setup(&mut self) -> AppResult<()> {
        Ok(())
    }

    fn process_task_response(
        &mut self,
        response: crate::app_ui::channel::TaskResponse,
    ) -> AppResult<()> {
        Ok(())
    }

    fn process_notification(&mut self, notification: &Notification) -> AppResult<()> {
        if let Notification::HelpText(ht) = notification {
            self.helptexts = ht.clone()
        }
        Ok(())
    }

    fn set_response(&mut self) {}

    fn get_common_state(&self) -> &CommonState {
        &self.common_state
    }

    fn get_common_state_mut(&mut self) -> &mut CommonState {
        &mut self.common_state
    }
}
