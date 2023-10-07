use ratatui::prelude::*;
use ratatui::style::Color;
use ratatui::widgets::{List, ListItem, Widget};

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
        if let Some(hovered) = self.cx.question.hovered() {
            return Some(
                self.cx
                    .question
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
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if let Some(ql) = self.get_questions_list() {
            let list = List::new(ql);
            list.render(area, buf);
            if self.cx.question.is_stats_visible() {
                stats::Stats::new(&self.cx.question).render(area, buf);
            }
        }
    }
}
