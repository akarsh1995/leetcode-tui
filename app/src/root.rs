use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget};

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
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let chunks = Layout::new()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(area);

        let _top_bar = chunks[0];
        let vert_center = chunks[1];
        let _bottom_bar = chunks[2];

        let center_chunks = Layout::new()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(vert_center);

        let topic_area = center_chunks[0];
        let question_area = center_chunks[1];

        Topic::new(self.cx).render(topic_area, buf);
        Questions::new(self.cx).render(question_area, buf);

        if self.cx.popup.visible {
            Popup::new(self.cx).render(area, buf);
        }
        if self.cx.select_popup.visible {
            SelectPopup::new(self.cx).render(area, buf);
        }

        if self.cx.input.visible {
            if let Some(input_text) = self.cx.input.text() {
                let _input_text = format!("/{input_text}");
                let line = Line::from(_input_text.as_str());
                Paragraph::new(line).render(_bottom_bar, buf);
            } else {
                let line = Line::from("/");
                Paragraph::new(line).render(_bottom_bar, buf);
            }
        }
    }
}
