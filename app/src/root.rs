use ratatui::prelude::*;
use ratatui::widgets::*;
use shared::layout::GetWindowStats;

use crate::ctx::Ctx;
use crate::help::Help;
use crate::popup::{Popup, SelectPopup};
use crate::question::Questions;
use crate::topic::Topic;

pub struct Root<'a> {
    cx: &'a mut Ctx,
}

impl<'a> Root<'a> {
    pub(super) fn new(cx: &'a mut Ctx) -> Self {
        Self { cx }
    }
}

impl<'a> Widget for Root<'a> {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if self.cx.help.is_visible() {
            Help::new(self.cx).render(_area, buf);
            return;
        }

        Topic::new(self.cx).render(_area, buf);
        Questions::new(self.cx).render(_area, buf);

        if self.cx.popup.visible {
            Popup::new(self.cx).render(_area, buf);
        }

        if self.cx.select_popup.visible {
            SelectPopup::new(self.cx).render(_area, buf);
        }

        if self.cx.input.visible {
            let mut search_text: String = "/".into();
            if let Some(input_text) = self.cx.input.text() {
                search_text.push_str(input_text);
            }
            let line = Line::from(search_text.as_str());
            Paragraph::new(line).render(self.get_window().root.status_bar, buf);
        }
    }
}
