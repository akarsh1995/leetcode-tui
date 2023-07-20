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
        (0, left_chunks[0]),
        (1, left_chunks[1]),
        (2, right_chunk[0]),
    ]);

    for (i, wid) in app.widgets().iter_mut().enumerate() {
        wid.render(layout_map.get(&(i as i32)).unwrap().clone(), f)
    }

    //         super::app::Widget::QuestionList(ql) => {
    //             let questions: Vec<ListItem> = ql
    //                 .items
    //                 .iter()
    //                 .map(|question| {
    //                     let mut lines = vec![];
    //                     if let Some(title) = &question.title {
    //                         lines.push(Line::from(format!(
    //                             "{:0>4}: {}",
    //                             question.frontend_question_id, title,
    //                         )));
    //                     }
    //                     ListItem::new(lines)
    //                 })
    //                 .collect();

    //             let items = List::new(questions)
    //                 .block(
    //                     Block::default()
    //                         .borders(Borders::ALL)
    //                         .title("Questions")
    //                         .border_style(border_style),
    //                 )
    //                 .highlight_style(
    //                     Style::default()
    //                         .bg(Color::White)
    //                         .fg(Color::Black)
    //                         .add_modifier(Modifier::BOLD),
    //                 )
    //                 .highlight_symbol(">> ");
    //             f.render_stateful_widget(items, right_chunk[0], &mut ql.state);

    //             let create_block = |title| {
    //                 Block::default()
    //                     .borders(Borders::ALL)
    //                     .style(Style::default().fg(Color::Gray))
    //                     .title(Span::styled(
    //                         title,
    //                         Style::default().add_modifier(Modifier::BOLD),
    //                     ))
    //             };

    //             let block = create_block("Stats");
    //             let inner_area = block.inner(left_chunks[1]);

    //             f.render_widget(block, left_chunks[1]);

    //             let stats = Stats { qm: &ql.items };

    //             let guage = |title: &'a str, val: usize, total: usize| {
    //                 let block_title = format!("{}: {}/{}", title, val, total);
    //                 let percentage = if total != 0 {
    //                     (val as f32 / total as f32) * 100_f32
    //                 } else {
    //                     0 as f32
    //                 };
    //                 let label = Span::styled(
    //                     format!("{:.2}%", percentage),
    //                     Style::default()
    //                         .fg(Color::White)
    //                         .add_modifier(Modifier::ITALIC | Modifier::BOLD),
    //                 );

    //                 Gauge::default()
    //                     .block(Block::default().title(block_title).borders(Borders::ALL))
    //                     .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
    //                     .percent(percentage as u16)
    //                     .label(label)
    //             };

    //             let horizontal_partition = Layout::default()
    //                 .direction(Direction::Horizontal)
    //                 .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    //                 .split(inner_area);

    //             let left_partition = Layout::default()
    //                 .direction(Direction::Vertical)
    //                 .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    //                 .split(horizontal_partition[0]);

    //             let right_partition = Layout::default()
    //                 .direction(Direction::Vertical)
    //                 .constraints([
    //                     Constraint::Percentage(33),
    //                     Constraint::Percentage(33),
    //                     Constraint::Percentage(33),
    //                 ])
    //                 .split(horizontal_partition[1]);

    //             f.render_widget(
    //                 guage(
    //                     "Attempted",
    //                     stats.get_total_question() - stats.get_not_attempted(),
    //                     stats.get_total_question(),
    //                 ),
    //                 left_partition[0],
    //             );
    //             f.render_widget(
    //                 guage("Accepted", stats.get_accepted(), stats.get_total_question()),
    //                 left_partition[1],
    //             );

    //             f.render_widget(
    //                 guage("Easy", stats.get_easy_accepted(), stats.get_easy_count()),
    //                 right_partition[0],
    //             );

    //             f.render_widget(
    //                 guage(
    //                     "Medium",
    //                     stats.get_medium_accepted(),
    //                     stats.get_medium_count(),
    //                 ),
    //                 right_partition[1],
    //             );

    //             f.render_widget(
    //                 guage("Hard", stats.get_hard_accepted(), stats.get_hard_count()),
    //                 right_partition[2],
    //             );
    //         }
    //     }
    // }

    // if app.show_popup {
    //     let mut popup_title = "".to_string();
    //     let mut popup_content = "".to_string();

    //     if let Some(response) = &app.last_response {
    //         match response {
    //             super::channel::TaskResponse::QuestionDetail(qd) => {
    //                 if let super::app::Widget::QuestionList(ql) = app.get_current_widget() {
    //                     popup_title = ql
    //                         .get_selected_item()
    //                         .as_ref()
    //                         .unwrap()
    //                         .title
    //                         .as_ref()
    //                         .unwrap()
    //                         .as_str()
    //                         .to_owned();
    //                     popup_content = qd.html_to_text();
    //                 };
    //             }
    //             super::channel::TaskResponse::Error(e) => {
    //                 popup_title = "Error".to_string();
    //                 popup_content = e.to_owned();
    //             }
    //             _ => {}
    //         }
    //     }
    //     handle_popup(app, f, popup_content.as_str(), popup_title.as_str())
    // }
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
