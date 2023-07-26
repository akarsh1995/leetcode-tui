use crate::{
    app_ui::{
        channel::ChannelRequestSender,
        components::{help_text::CommonHelpText, popups::Component, rect::centered_rect},
    },
    errors::AppResult,
};

use crossterm::event::KeyEvent;

use ratatui::prelude::*;

use super::{
    notification::{NotifContent, Notification, PopupType, WidgetName},
    CommonState, CrosstermStderr, Widget,
};

#[derive(Debug)]
pub(crate) struct Popup {
    pub common_state: CommonState,
    // pub message: String,
    pub callee_wid: Option<WidgetName>,
    pub popup_type: Option<PopupType>,
}

impl Popup {
    pub fn new(widget_name: WidgetName, task_sender: ChannelRequestSender) -> Self {
        Self {
            common_state: CommonState::new(
                widget_name,
                task_sender,
                vec![CommonHelpText::Close.into()],
            ),
            popup_type: None,
            callee_wid: None,
        }
    }
}

impl Widget for Popup {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        if self.is_active() {
            let size = rect;
            let size = centered_rect(60, 50, size);
            if let Some(pt) = &self.popup_type {
                match pt {
                    PopupType::Paragraph(p) => p.render(frame, size),
                }
            }
        }
    }

    fn handler(&mut self, event: KeyEvent) -> AppResult<Option<Notification>> {
        match event.code {
            crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Esc => {
                self.set_inactive()
            }
            _ => {
                let fwd_event_to_parent = match &mut self.popup_type {
                    Some(popup) => match popup {
                        PopupType::Paragraph(p) => p.event_handler(event),
                    },
                    None => None,
                };

                // in case popup has helptext to take the event but does not have associated key in
                // the handler mapping pass it to the popup callee
                if let Some(fwd_evt) = fwd_event_to_parent {
                    return Ok(Some(Notification::Event(NotifContent {
                        src_wid: self.get_widget_name(),
                        dest_wid: self.callee_wid.as_ref().unwrap().clone(),
                        content: fwd_evt,
                    })));
                }
            }
        }
        Ok(None)
    }

    fn process_notification(
        &mut self,
        notification: Notification,
    ) -> AppResult<Option<Notification>> {
        if let Notification::Popup(NotifContent {
            src_wid,
            dest_wid: _,
            content,
        }) = notification
        {
            self.callee_wid = Some(src_wid);
            let extended_help = match &content.popup {
                PopupType::Paragraph(p) => p.get_key_set(),
            };
            self.popup_type = Some(content.popup);
            self.get_help_texts_mut().extend(content.help_texts.clone());
            self.get_help_texts_mut().extend(extended_help);
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

    fn get_notification_queue(&mut self) -> &mut std::collections::VecDeque<Notification> {
        &mut self.common_state.notification_queue
    }
}
