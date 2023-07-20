use std::collections::HashMap;

use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use super::{app::App, widgets::CrosstermStderr};

/// Renders the user interface widgets.
pub fn render(app: &mut App, f: &mut CrosstermStderr) {
    // Create two chunks with equal horizontal screen space
    let size = f.size();

    let terminal_main_block = Block::default()
        .borders(Borders::ALL)
        .title("Leetcode TUI")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let inner_size = terminal_main_block.inner(f.size());

    f.render_widget(terminal_main_block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(inner_size);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let right_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[1]);

    let layout_map = HashMap::from([
        (0, left_chunks[0]), // tags
        (1, right_chunk[0]),
        (2, left_chunks[1]), // question
    ]);

    if app.has_popups() {
        if let Some(top_popup) = app.popups.last_mut() {
            top_popup.render(inner_size, f);
            return;
        }
    }

    for (i, wid) in app.widgets().iter_mut().enumerate() {
        wid.render(*layout_map.get(&(i as i32)).unwrap(), f)
    }
}
