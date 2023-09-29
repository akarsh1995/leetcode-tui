use ratatui::prelude::*;
use ratatui::style::Color;
use ratatui::widgets::{List, ListItem, Widget};

use crate::ctx::Ctx;

pub struct Topic<'a> {
    cx: &'a Ctx,
}

impl<'a> Topic<'a> {
    pub(super) fn new(cx: &'a Ctx) -> Self {
        Self { cx }
    }
}

impl<'a> Widget for Topic<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if let Some(hovered) = self.cx.topic.hovered() {
            let lines = self
                .cx
                .topic
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
            let list = List::new(lines);
            list.render(area, buf);
        }
    }
}
