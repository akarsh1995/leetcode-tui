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

    for (i, wid) in app.widgets().iter_mut().enumerate() {
        wid.render(layout_map.get(&(i as i32)).unwrap().clone(), f)
    }
}

pub fn handle_popup<B: Backend>(
    app: &mut App,
    f: &mut Frame<'_, B>,
    popup_msg: &str,
    question_title: &str,
) {
    let size = f.size();

    let text = if app.show_popup {
        "Press esc to close the question info"
    } else {
        "Press ↵  to show the question info"
    };

    // top message press p to close
    let paragraph = Paragraph::new(text.slow_blink())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, size);

    if app.show_popup {
        let create_block = |title| {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray))
                .title(Span::styled(
                    title,
                    Style::default().add_modifier(Modifier::BOLD),
                ))
        };

        let block = create_block(question_title);
        let area = centered_rect(60, 100, size);
        let inner = block.inner(area);
        f.render_widget(Clear, area); //this clears out the background
                                      // f.render_widget(block.clone(), area);

        let content = Paragraph::new(popup_msg)
            .wrap(Wrap { trim: true })
            .block(block);

        f.render_widget(content, inner);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
