use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget};
use shared::layout::GetWindowStats;

use crate::ctx::Ctx;
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
        Topic::new(self.cx).render(_area, buf);
        Questions::new(self.cx).render(_area, buf);

        if self.cx.popup.visible {
            Popup::new(self.cx).render(_area, buf);
        }
        if self.cx.select_popup.visible {
            SelectPopup::new(self.cx).render(_area, buf);
        }

        if self.cx.input.visible {
            if let Some(input_text) = self.cx.input.text() {
                let _input_text = format!("/{input_text}");
                let line = Line::from(_input_text.as_str());
                Paragraph::new(line).render(self.get_window().root.status_bar, buf);
            } else {
                let line = Line::from("/");
                Paragraph::new(line).render(self.get_window().root.status_bar, buf);
            }
        }
    }
}
