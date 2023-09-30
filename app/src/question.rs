use ratatui::prelude::*;
use ratatui::style::Color;
use ratatui::widgets::{List, ListItem, Widget};

use crate::ctx::Ctx;

pub struct Questions<'a> {
    cx: &'a Ctx,
}

impl<'a> Questions<'a> {
    pub(super) fn new(cx: &'a Ctx) -> Self {
        Self { cx }
    }
}

impl<'a> Widget for Questions<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if let Some(hovered) = self.cx.question.hovered() {
            let lines = self
                .cx
                .question
                .window()
                .iter()
                .map(|q| {
                    ListItem::new(q.title.as_str())
                        .bg(if q.id == hovered.id {
                            Color::White
                        } else {
                            Color::default()
                        })
                        .fg(if q.id == hovered.id {
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
