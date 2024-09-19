use leetcode_tui_config::CONFIG;
use leetcode_tui_shared::layout::GetWindowStats;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Widget};

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
        q: &'b leetcode_tui_db::DbQuestion,
        hovered: &'b leetcode_tui_db::DbQuestion,
    ) -> ListItem<'b> {
        let config = &CONFIG.as_ref().theme.question;
        let c_hovered = &config.hovered;
        let normal = &config.normal;
        let easy_hovered = c_hovered.easy;
        let medium_hovered = c_hovered.medium;
        let hard_hovered = c_hovered.hard;
        let easy = normal.easy;
        let medium = normal.medium;
        let hard = normal.hard;

        ListItem::new(q.to_string()).style(if q.id == hovered.id {
            if q.is_easy() {
                easy_hovered
            } else if q.is_medium() {
                medium_hovered
            } else {
                hard_hovered
            }
        } else {
            if q.is_easy() {
                easy
            } else if q.is_medium() {
                medium
            } else {
                hard
            }
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
            .border_style(CONFIG.as_ref().theme.border.hovered)
            .cyan()
            .title("Questions")
            .title_alignment(Alignment::Center);

        let term_window = self.get_window();

        q_area_surrounding_block.render(term_window.root.center_layout.question.outer, buf);

        if let Some(ql) = self.get_questions_list() {
            let list = List::new(ql);
            Widget::render(list, term_window.root.center_layout.question.inner, buf);
            if self.cx.content.get_questions().is_stats_visible() {
                stats::Stats::new(&self.cx.content.get_questions())
                    .render(term_window.root.q_stats.outer, buf);
            }
        }
    }
}
