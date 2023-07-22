use std::collections::HashMap;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
};

use super::{
    app::App,
    widgets::{notification::WidgetName, CrosstermStderr, Widget},
};

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
        (WidgetName::TopicList, left_chunks[0]),    // tags
        (WidgetName::Stats, left_chunks[1]),        // stats
        (WidgetName::QuestionList, right_chunk[0]), // question
        (WidgetName::HelpLine, size),
    ]);

    for (name, wid) in app.widget_map.iter_mut() {
        let rect = layout_map.get(name).unwrap();
        wid.render(*rect, f);
    }

    if let Some(popup) = app.get_current_popup_mut() {
        return popup.render(size, f);
    }
}
