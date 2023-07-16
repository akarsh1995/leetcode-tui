use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use super::app::App;

/// Renders the user interface widgets.
pub fn render<'a, B: Backend>(app: &'a mut App, f: &mut Frame<'_, B>) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Iterate through all elements in the `items` app and append some debug text to it.
    for (i, w) in app.widgets.iter_mut().enumerate() {
        let is_widget_active = app.widget_switcher as usize == i;
        let mut border_style = Style::default();
        if is_widget_active {
            border_style = border_style.fg(Color::Cyan);
        }
        match w {
            super::app::Widget::TopicTagList(ttl) => {
                let items: Vec<ListItem> = ttl
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
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Tags")
                            .border_style(border_style),
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                // We can now render the item list
                f.render_stateful_widget(items, chunks[0], &mut ttl.state);
            }
            super::app::Widget::QuestionList(ql) => {
                let questions: Vec<ListItem> = ql
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
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Questions")
                            .border_style(border_style),
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");
                f.render_stateful_widget(items, chunks[1], &mut ql.state);
            }
        }
    }
}
