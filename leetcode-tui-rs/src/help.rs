use ratatui::prelude::*;
use ratatui::widgets::Widget;
use ratatui::widgets::*;

use crate::ctx::Ctx;

pub(super) struct Help<'a> {
    cx: &'a mut Ctx,
}

impl<'a> Help<'a> {
    pub(super) fn new(cx: &'a mut Ctx) -> Self {
        Self { cx }
    }
}

impl<'a> Widget for Help<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(229, 228, 226))
            .fg(Color::Black);
        let rows = self.cx.help.get_items().iter().map(|item| {
            let cells = item.iter().map(|c| Cell::from(*c));
            Row::new(cells)
        });

        let t = Table::new(
            rows,
            [
                Constraint::Percentage(50),
                Constraint::Max(30),
                Constraint::Min(10),
            ],
        )
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .highlight_style(selected_style);
        Clear.render(area, buf);
        ratatui::widgets::StatefulWidget::render(t, area, buf, self.cx.help.get_mut_state());
    }
}
