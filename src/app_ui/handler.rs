use std::collections::HashSet;

use super::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::ListState;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let curr_widget = &mut app.widgets[app.widget_switcher as usize];
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Up => match curr_widget {
            super::app::Widget::QuestionList(ql) => ql.previous(),
            super::app::Widget::TopicTagList(tt) => tt.previous(),
        },
        KeyCode::Down => match curr_widget {
            super::app::Widget::QuestionList(ql) => ql.next(),
            super::app::Widget::TopicTagList(tt) => tt.next(),
        },
        KeyCode::Left => app.prev_widget(),
        KeyCode::Right => app.next_widget(),
        // Other handlers you could add here.
        _ => {}
    }
    app.update_list();

    // post key event update the question list
    Ok(())
}
