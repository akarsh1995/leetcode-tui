use crate::{
    app_ui::{
        channel::ChannelRequestSender,
        components::{popups::Component, rect::centered_rect},
    },
    errors::AppResult,
};

use crossterm::event::{KeyCode, KeyEvent};

use ratatui::prelude::*;

use super::{
    notification::{NotifContent, Notification, PopupType, WidgetName},
    CommonState, CrosstermStderr, Widget,
};

#[derive(Debug)]
pub(crate) struct Popup {
    pub common_state: CommonState,
    pub callee_wid: Option<WidgetName>,
    pub popup_type: Option<PopupType>,
}

impl Popup {
    pub fn new(widget_name: WidgetName, task_sender: ChannelRequestSender) -> Self {
        Self {
            common_state: CommonState::new(widget_name, task_sender, vec![]),
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
            if let Some(pt) = &mut self.popup_type {
                match pt {
                    PopupType::Paragraph(p) => p.render(frame, size),
                    PopupType::List { popup: l, .. } => l.render(frame, size),
                }
            }
        }
    }

    fn handler(&mut self, event: KeyEvent) -> AppResult<Option<Notification>> {
        let src_wid = self.get_widget_name();
        if let Some(popup) = &mut self.popup_type {
            match popup {
                PopupType::Paragraph(para_popup) => {
                    let notif: Option<KeyEvent> = para_popup.event_handler(event);

                    if !para_popup.is_showing() {
                        self.set_inactive();
                    }

                    if let Some(n) = notif {
                        if self.can_handle_key_set().contains(&n.code) {
                            return Ok(Some(Notification::Event(NotifContent {
                                src_wid,
                                dest_wid: self.callee_wid.as_ref().unwrap().clone(),
                                content: n,
                            })));
                        }
                    }
                }
                PopupType::List { popup, key } => {
                    let mut notif = None;
                    if let Some(key_event) = popup.event_handler(event) {
                        if let KeyCode::Enter = key_event.code {
                            let i = popup.get_selected_index();
                            notif = Some(Notification::SelectedItem(NotifContent {
                                src_wid,
                                dest_wid: self.callee_wid.as_ref().unwrap().clone(),
                                content: (key.to_string(), i),
                            }));
                        }
                    }
                    if !popup.is_showing() {
                        self.set_inactive();
                    }
                    return Ok(notif);
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
                PopupType::List { popup: l, .. } => l.get_key_set(),
            };
            self.popup_type = Some(content.popup);
            self.get_help_texts_mut().extend(content.help_texts.clone());
            // extend help specific to the popup recieved
            self.get_help_texts_mut().extend(extended_help);
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
