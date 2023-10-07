use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Gauge, Widget};

pub(super) struct Stats<'a> {
    cx: &'a app_core::question::Questions,
}

impl<'a> Stats<'a> {
    pub(super) fn new(quests: &'a app_core::question::Questions) -> Self {
        Self { cx: quests }
    }
}

fn create_gauge(title: &str, val: usize, total: usize, style: Style) -> Gauge {
    let block_title = format!("{}: {}/{}", title, val, total);
    let percentage = if total != 0 {
        (val as f32 / total as f32) * 100_f32
    } else {
        0 as f32
    };
    let label = Span::styled(
        format!("{:.2}%", percentage),
        style
            .add_modifier(Modifier::ITALIC | Modifier::BOLD)
            .fg(Color::White),
    );

    Gauge::default()
        .block(Block::default().title(block_title).borders(Borders::ALL))
        .gauge_style(style.fg(Color::Gray))
        .percent(percentage as u16)
        .label(label)
}

impl<'a> Stats<'a> {
    fn create_block(title: &str) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    }
}

impl<'a> Widget for Stats<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Self::create_block("Stats");
        let inner_area = block.inner(area);
        block.render(area, buf);

        let horizontal_partition = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner_area);

        let left_partition = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(horizontal_partition[0]);

        let right_partition = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(horizontal_partition[1]);

        Clear.render(area, buf);

        for ((title, numerator, denominator), render_area) in
            self.cx.get_stats().get_ratios().into_iter().zip(
                [
                    left_partition[0],
                    left_partition[1],
                    right_partition[0],
                    right_partition[1],
                    right_partition[2],
                ]
                .iter(),
            )
        {
            create_gauge(title, numerator, denominator, Style::default()).render(*render_area, buf)
        }
    }
}
