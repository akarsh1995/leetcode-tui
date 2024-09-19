use leetcode_tui_config::CONFIG;
use leetcode_tui_shared::layout::GetWindowStats;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Widget};

use crate::ctx::Ctx;

pub struct Topic<'a> {
    cx: &'a Ctx,
}

impl<'a> Topic<'a> {
    pub(super) fn new(cx: &'a Ctx) -> Self {
        Self { cx }
    }

    fn get_styled_block(&self) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .border_style(CONFIG.as_ref().theme.border.normal)
            .cyan()
            .title("Topics")
            .title_alignment(Alignment::Center)
    }
}

impl<'a> Widget for Topic<'a> {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if let Some(hovered) = self.cx.content.get_topic().hovered() {
            let config = &CONFIG.as_ref().theme.topic;
            let c_hovered = config.hovered;
            let normal = config.normal.into();

            let lines = self
                .cx
                .content
                .get_topic()
                .window()
                .iter()
                .map(|t| {
                    ListItem::new(t.slug.as_str()).style(if t.slug == hovered.slug {
                        c_hovered
                    } else {
                        normal
                    })
                })
                .collect::<Vec<_>>();
            self.get_styled_block()
                .render(self.get_window().root.center_layout.topic.outer, buf);
            let list = List::new(lines);
            Widget::render(list, self.get_window().root.center_layout.topic.inner, buf);
        }
    }
}
