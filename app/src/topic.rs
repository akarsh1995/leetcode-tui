use ratatui::prelude::*;
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, List, ListItem, Widget};
use shared::layout::GetWindowStats;

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
            .cyan()
            .title("Topics")
            .title_alignment(Alignment::Center)
    }
}

impl<'a> Widget for Topic<'a> {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if let Some(hovered) = self.cx.content.get_topic().hovered() {
            let lines = self
                .cx
                .content
                .get_topic()
                .window()
                .iter()
                .map(|t| {
                    ListItem::new(t.slug.as_str())
                        .bg(if t.slug == hovered.slug {
                            Color::White
                        } else {
                            Color::default()
                        })
                        .fg(if t.slug == hovered.slug {
                            Color::LightYellow
                        } else {
                            Color::default()
                        })
                })
                .collect::<Vec<_>>();
            self.get_styled_block()
                .render(self.get_window().root.center_layout.topic.outer, buf);
            let list = List::new(lines);
            list.render(self.get_window().root.center_layout.topic.inner, buf);
        }
    }
}
