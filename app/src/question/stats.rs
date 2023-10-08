use app_core::content;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Gauge, Widget};
use shared::layout::GetWindowStats;

pub(super) struct Stats<'a> {
    cx: &'a content::question::Questions,
}

impl<'a> Stats<'a> {
    pub(super) fn new(quests: &'a content::question::Questions) -> Self {
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
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center)
            .cyan()
    }
}

impl<'a> Widget for Stats<'a> {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Self::create_block("Stats");

        block.render(self.get_window().root.q_stats.outer, buf);

        Clear.render(self.get_window().root.q_stats.inner, buf);
        let horizontal_partition = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(self.get_window().root.q_stats.inner);

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
