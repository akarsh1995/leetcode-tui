use crate::ctx::Ctx;
use config::CONFIG;
use ratatui::prelude::*;
use ratatui::widgets::{
    Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar, StatefulWidget, Widget, Wrap,
};
use shared::layout::GetWindowStats;

pub struct SelectPopup<'a> {
    ctx: &'a mut Ctx,
}

impl<'a> SelectPopup<'a> {
    pub fn new(ctx: &'a mut Ctx) -> Self {
        Self { ctx }
    }
}

impl<'a> Widget for SelectPopup<'a> {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        let c_def = &CONFIG.as_ref().theme.defaults;
        let mut block: Block<'_> = Block::default();
        if let Some(title) = self.ctx.select_popup.get_title() {
            block = Block::default().title(title);
        }
        let block = block
            .borders(Borders::ALL)
            .border_style(Style::default().fg(c_def.info.into()));
        Clear.render(self.get_window().root.popup.outer, buf);
        block.render(self.get_window().root.popup.outer, buf);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(100), Constraint::Min(1)])
            .split(self.get_window().root.popup.inner);
        let content_area = chunks[0];
        let lines = self.ctx.select_popup.get_lines().clone();

        let list = List::new(
            lines
                .iter()
                .map(|l| ListItem::new(vec![Line::from(l.as_ref())]).fg(c_def.fg_dark))
                .collect::<Vec<_>>(),
        )
        .highlight_style(
            Style::default()
                .bg(c_def.bg_highlight.into())
                .fg(c_def.fg.into())
                .add_modifier(Modifier::BOLD),
        );
        StatefulWidget::render(list, content_area, buf, &mut self.ctx.select_popup.state);
        // Scrollbar::default()
        //     .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
        //     .begin_symbol(Some("↑"))
        //     .end_symbol(Some("↓"))
        //     .render(scrollbar_area, buf, &mut self.ctx.popup.v_scroll_state)
    }
}

pub struct Popup<'a> {
    ctx: &'a mut Ctx,
}

impl<'a> Popup<'a> {
    pub fn prepare_lines(&self) -> Vec<Line> {
        self.ctx
            .popup
            .get_lines()
            .iter()
            .map(|l| Line::from(l.as_str()))
            .collect()
    }

    pub fn prepare_paragraph(&self) -> Paragraph<'_> {
        Paragraph::new(self.prepare_lines())
            .scroll((self.ctx.popup.v_scroll, 0))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(CONFIG.as_ref().theme.defaults.fg.into()))
    }
}

impl<'a> Popup<'a> {
    pub fn new(ctx: &'a mut Ctx) -> Self {
        Self { ctx }
    }
}

impl<'a> Widget for Popup<'a> {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let c_def = &CONFIG.as_ref().theme.defaults;
        let mut block: Block<'_> = Block::default();
        if let Some(title) = self.ctx.popup.get_title() {
            block = Block::default().title(title);
        }
        let block = block
            .borders(Borders::ALL)
            .border_style(Style::default().fg(c_def.info.into()));
        Clear.render(self.get_window().root.popup.outer, buf);
        block.render(self.get_window().root.popup.outer, buf);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(100), Constraint::Min(1)])
            .split(self.get_window().root.popup.inner);
        let content_area = chunks[0];
        let scrollbar_area = chunks[1];
        self.prepare_paragraph().render(content_area, buf);
        Scrollbar::default()
            .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .render(scrollbar_area, buf, &mut self.ctx.popup.v_scroll_state)
    }
}
