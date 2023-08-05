use crossterm::event::{KeyCode, KeyEvent};
use indexmap::IndexSet;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem},
};

use crate::app_ui::components::{help_text::CommonHelpText, list::StatefulList};

use super::{CommonState, Component, FrameBackend};

#[derive(Debug, Clone)]
pub(crate) struct SelectionListPopup {
    common_state: CommonState,
    list: StatefulList<String>,
}

impl SelectionListPopup {
    pub fn new(title: String, list_items: Vec<String>) -> Self {
        Self {
            common_state: CommonState {
                help_text: IndexSet::from_iter([
                    CommonHelpText::Close.into(),
                    CommonHelpText::ScrollUp.into(),
                    CommonHelpText::ScrollDown.into(),
                    CommonHelpText::Select.into(),
                ]),
                title,
                show: true,
            },
            list: StatefulList::with_items(list_items),
        }
    }

    pub fn get_selected_index(&self) -> usize {
        self.list.state.selected().unwrap()
    }
}

impl Component for SelectionListPopup {
    // selection list specific events
    fn event_handler(&mut self, event: KeyEvent) -> Option<KeyEvent> {
        match event.code {
            KeyCode::Esc => self.hide(),
            KeyCode::Up => self.list.previous(),
            KeyCode::Down => self.list.next(),
            // only escape key get passed to the parent event
            KeyCode::Enter => {
                self.hide();
                return Some(event);
            }
            _ => return Some(event),
        }
        None
    }

    fn render(&mut self, f: &mut FrameBackend, render_area: Rect) {
        let lines = self
            .list
            .items
            .iter()
            .map(|item| {
                let line_text = item;
                ListItem::new(Span::styled(line_text, Style::default()))
            })
            .collect::<Vec<_>>();

        let border_style = Style::default().fg(Color::Cyan);
        let items = List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.common_state.title.as_ref())
                    .border_style(border_style),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Rgb(0, 0, 0))
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(Clear, render_area);
        f.render_stateful_widget(items, render_area, &mut self.list.state);
    }

    fn get_common_state_mut(&mut self) -> &mut CommonState {
        &mut self.common_state
    }

    fn get_common_state(&self) -> &CommonState {
        &self.common_state
    }
}
