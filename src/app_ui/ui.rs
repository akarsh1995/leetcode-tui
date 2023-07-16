// use ratatui::{
//     backend::Backend,
//     layout::{Alignment, Layout},
//     style::{Color, Style},
//     widgets::{Block, BorderType, Borders, Paragraph},
//     Frame,
// };
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

use super::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, f: &mut Frame<'_, B>) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .topic_tags_stateful
        .items
        .iter()
        .map(|tt_model| {
            if let Some(name) = &tt_model.name {
                let lines = vec![Line::from(name.as_str())];
                ListItem::new(lines)
            } else {
                ListItem::new(vec![Line::from("")])
            }
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.topic_tags_stateful.state);

    let questions: Vec<ListItem> = app
        .questions_stateful
        .items
        .iter()
        .map(|question| {
            let lines = vec![Line::from(format!(
                "{}: {:?}",
                question.frontend_question_id, question.title
            ))];
            ListItem::new(lines)
        })
        .collect();

    let items = List::new(questions)
        .block(Block::default().borders(Borders::ALL).title("Questions"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, chunks[1], &mut app.questions_stateful.state);
}
