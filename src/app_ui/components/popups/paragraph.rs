use crossterm::event::{KeyCode, KeyEvent};
use indexmap::IndexSet;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app_ui::components::help_text::CommonHelpText;

use super::{CommonState, Component, FrameBackend};

#[derive(Debug, Clone)]
pub(crate) struct ParagraphPopup {
    common_state: CommonState,
    content: String,
    pub scroll_x: u16,
    pub scroll_y: u16,
}

impl ParagraphPopup {
    pub fn new(title: String, content: String) -> Self {
        Self {
            common_state: CommonState {
                help_text: IndexSet::from_iter([
                    CommonHelpText::Close.into(),
                    CommonHelpText::ScrollUp.into(),
                    CommonHelpText::ScrollDown.into(),
                ]),
                title,
            },
            content,
            scroll_x: 0,
            scroll_y: 0,
        }
    }
}

impl Component for ParagraphPopup {
    fn event_handler(&mut self, event: KeyEvent) -> Option<KeyEvent> {
        match event.code {
            KeyCode::Up => self.scroll_y = self.scroll_y.saturating_sub(1),
            KeyCode::Down => self.scroll_y += 1,
            _ => return Some(event),
        }
        None
    }

    fn render(&self, f: &mut FrameBackend, render_area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                self.common_state.title.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ));

        let content = Paragraph::new(self.content.as_str())
            .wrap(Wrap { trim: true })
            .scroll((self.scroll_y, self.scroll_x))
            .block(block);

        f.render_widget(Clear, render_area);
        f.render_widget(content, render_area); // frame.render_widget(block, area);
    }

    fn get_common_state_mut(&mut self) -> &mut CommonState {
        &mut self.common_state
    }

    fn get_common_state(&self) -> &CommonState {
        &self.common_state
    }
}
