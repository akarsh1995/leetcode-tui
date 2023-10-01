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
    use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

    use crate::ctx::Ctx;

    pub struct Popup<'a> {
        ctx: &'a Ctx,
    }

    impl<'a> Popup<'a> {
        pub fn new(ctx: &'a Ctx) -> Self {
            Self { ctx }
        }
    }

    impl<'a> Widget for Popup<'a> {
        fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
            let block = Block::default().title("Popup").borders(Borders::ALL);
            let area = centered_rect(60, 20, area);
            let lines = self.ctx.popup.get_text();
            let joined = lines.join("\n");
            Clear.render(area, buf);
            let inner = block.inner(area).clone();
            block.render(area, buf);
            Paragraph::new(joined).render(inner, buf);
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
