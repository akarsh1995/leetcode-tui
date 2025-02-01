use crate::ctx::Ctx;
use leetcode_tui_config::CONFIG;
use ratatui::prelude::*;
use ratatui::widgets::{Gauge, Widget};

pub(super) struct ProgressBar<'a> {
    cx: &'a Ctx,
}

impl<'a> ProgressBar<'a> {
    pub fn new(cx: &'a Ctx) -> Self {
        Self { cx }
    }

    fn create_gauge_without_block(title: &str, val: usize, total: usize, style: Style) -> Gauge {
        let percentage = if total != 0 {
            (val as f32 / total as f32) * 100_f32
        } else {
            0 as f32
        };
        let label = Span::styled(
            format!("{title} - {:.2}%", percentage),
            style
                .add_modifier(Modifier::ITALIC | Modifier::BOLD)
                .fg(Color::White)
                .bg(Color::Black),
        );

        Gauge::default()
            .gauge_style(style)
            .percent(percentage as u16)
            .label(label)
    }
}

impl<'a> Widget for ProgressBar<'a> {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let progress = &self.cx.progress;

        let style = CONFIG.as_ref().theme.question.normal.easy.into();
        let progress_guage = Self::create_gauge_without_block(
            progress.get_title(),
            progress.get_progress() as usize,
            progress.get_total() as usize,
            style,
        );

        progress_guage.render(_area, buf);
    }
}
