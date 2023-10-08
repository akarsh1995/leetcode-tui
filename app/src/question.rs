use ratatui::prelude::*;
use ratatui::style::Color;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Widget};
use shared::layout::GetWindowStats;

use crate::ctx::Ctx;

pub(crate) mod stats;

pub struct Questions<'a> {
    cx: &'a Ctx,
}

impl<'a> Questions<'a> {
    pub(super) fn new(cx: &'a Ctx) -> Self {
        Self { cx }
    }

    fn prepare_list_item<'b>(
        &self,
        q: &'b leetcode_db::DbQuestion,
        hovered: &'b leetcode_db::DbQuestion,
    ) -> ListItem<'b> {
        ListItem::new(q.to_string())
            .bg(if q.id == hovered.id {
                Color::Green
            } else {
                Color::default()
            })
            .fg(if q.id == hovered.id {
                Color::White
            } else {
                Color::default()
            })
    }

    fn get_questions_list(&self) -> Option<Vec<ListItem<'_>>> {
        if let Some(hovered) = self.cx.content.get_questions().hovered() {
            return Some(
                self.cx
                    .content
                    .get_questions()
                    .window()
                    .iter()
                    .map(|q| self.prepare_list_item(q, hovered))
                    .collect::<Vec<_>>(),
            );
        }
        None
    }
}

impl<'a> Widget for Questions<'a> {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let q_area_surrounding_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .cyan()
            .title("Questions")
            .title_alignment(Alignment::Center);

        let term_window = self.get_window();

        q_area_surrounding_block.render(term_window.root.center_layout.question.outer, buf);

        if let Some(ql) = self.get_questions_list() {
            let list = List::new(ql);
            list.render(term_window.root.center_layout.question.inner, buf);
            if self.cx.content.get_questions().is_stats_visible() {
                stats::Stats::new(&self.cx.content.get_questions())
                    .render(term_window.root.q_stats.outer, buf);
            }
        }
    }
}
