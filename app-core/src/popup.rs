use ratatui::widgets::{ListState, ScrollbarState};
use std::fmt::Display;

use crate::emit;

#[derive(Default)]
pub struct Popup {
    pub visible: bool,
    lines: Vec<String>,
    pub v_scroll_state: ScrollbarState,
    pub v_scroll: u16,
    title: Option<String>,
}

impl Popup {
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_ref().map(|v| v.as_str())
    }
}

impl Popup {
    pub fn new(lines: Vec<String>) -> Self {
        let mut p = Popup {
            lines,
            ..Default::default()
        };
        p.v_scroll_state = p.v_scroll_state.content_length(p.lines.len() as u16);
        p
    }

    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        true
    }

    pub fn get_text(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn reset(&mut self, title: Option<String>, lines: Vec<String>) {
        let mut p = Self::new(lines);
        p.visible = self.visible;
        p.title = title;
        *self = p;
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn scroll_down(&mut self) -> bool {
        if self.v_scroll == self.lines.len().saturating_sub(1) as u16 {
            return false;
        }
        self.v_scroll = self.v_scroll.saturating_add(1);
        self.v_scroll_state = self.v_scroll_state.position(self.v_scroll);
        true
    }

    pub fn scroll_up(&mut self) -> bool {
        if self.v_scroll == 0 {
            return false;
        }
        self.v_scroll = self.v_scroll.saturating_sub(1);
        self.v_scroll_state = self.v_scroll_state.position(self.v_scroll);
        true
    }
}

#[derive(Default)]
pub struct SelectPopup<T: Display> {
    pub visible: bool,
    pub state: ListState,
    items: Vec<T>,
    sender: Option<tokio::sync::oneshot::Sender<Option<usize>>>,
    title: Option<String>,
}

impl<T: Display> SelectPopup<T> {
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_ref().map(|v| v.as_str())
    }
}

impl<T: Display> SelectPopup<T> {
    pub fn with_items(
        &mut self,
        maybe_title: Option<String>,
        items: Vec<T>,
        sender: tokio::sync::oneshot::Sender<Option<usize>>,
    ) {
        *self = SelectPopup {
            visible: self.visible,
            state: ListState::default(),
            items,
            sender: Some(sender),
            title: maybe_title,
        };
        if !self.items.is_empty() {
            self.state.select(Some(0))
        }
    }

    pub fn get_lines(&self) -> &Vec<T> {
        &self.items
    }

    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        true
    }

    pub fn close_unselected(&mut self) -> bool {
        if let Some(sender) = self.sender.take() {
            if let Err(e) = sender.send(None) {
                emit!(Error(format!("Could not send {e:?} via one shot channel.")));
            } else {
                self.toggle();
            }
        }
        true
    }

    pub fn close(&mut self) -> bool {
        let mut error_message = None;
        if let Some(sender) = self.sender.take() {
            let k = sender.send(self.state.selected());
            if let Err(e) = k {
                error_message = Some(format!(
                    "index: {:?} could not be sent through the channel",
                    e
                ));
            };
        } else {
            error_message = Some(
                "Sender not present in Stateful list. Cannot send the selected item.".to_string(),
            );
        }
        if let Some(em) = error_message {
            emit!(Error(em));
        }
        self.toggle();
        true
    }

    pub fn next_item(&mut self) -> bool {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        true
    }

    pub fn prev_item(&mut self) -> bool {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        true
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
