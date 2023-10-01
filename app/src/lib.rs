pub mod app;
pub mod ctx;
pub mod executor;
pub mod root;
pub mod signals;

pub mod question;
pub mod topic;
pub mod utils;

pub mod popup {

    // use crossterm::terminal::Clear;
    use ratatui::prelude::*;
    use ratatui::widgets::{
        Block, Borders, Clear, Paragraph, Scrollbar, StatefulWidget, Widget, Wrap,
    };

    use crate::ctx::Ctx;

    pub struct Popup<'a> {
        ctx: &'a mut Ctx,
    }

    impl<'a> Popup<'a> {
        pub fn prepare_lines(&self) -> Vec<Line> {
            self.ctx
                .popup
                .get_lines()
                .iter()
                .map(|l| Line::from(l.as_str()))
                .collect()
        }

        pub fn prepare_paragraph(&self) -> Paragraph<'_> {
            Paragraph::new(self.prepare_lines())
                .scroll((self.ctx.popup.v_scroll, 0))
                .wrap(Wrap { trim: true })
        }
    }

    impl<'a> Popup<'a> {
        pub fn new(ctx: &'a mut Ctx) -> Self {
            Self { ctx }
        }
    }

    impl<'a> Widget for Popup<'a> {
        fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
            let block = Block::default().title("Popup").borders(Borders::ALL);
            let area = centered_rect(60, 60, area);
            Clear.render(area, buf);
            let inner = block.inner(area);
            block.render(area, buf);
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(100), Constraint::Min(1)])
                .split(inner);
            let content_area = chunks[0];
            let scrollbar_area = chunks[1];
            self.prepare_paragraph().render(content_area, buf);
            Scrollbar::default()
                .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"))
                .render(scrollbar_area, buf, &mut self.ctx.popup.v_scroll_state)
        }
    }

    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }
}
