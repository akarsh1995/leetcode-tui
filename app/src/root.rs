use ratatui::prelude::*;
use ratatui::widgets::Widget;

use crate::ctx::Ctx;
use crate::topic::Topic;

pub struct Root<'a> {
    cx: &'a Ctx,
}

impl<'a> Root<'a> {
    pub(super) fn new(cx: &'a Ctx) -> Self {
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

        let top_bar = chunks[0];
        let vert_center = chunks[1];
        let bottom_bar = chunks[2];
        Topic::new(self.cx).render(vert_center, buf);
    }
}
