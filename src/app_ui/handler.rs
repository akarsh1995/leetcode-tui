use super::{
    app::{App, Widget},
    channel,
};
use crate::errors::AppResult;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let curr_widget = &mut app.widgets[app.widget_switcher as usize];
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Esc => {
            if app.show_popup {
                app.toggle_popup();
            }
        }

        KeyCode::Enter => {
            if let Widget::QuestionList(_) = app.get_current_widget() {
                app.toggle_popup();
                app.update_question_in_popup()?;
            }
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.toggle_popup();
        }
        // Counter handlers
        KeyCode::Up => match curr_widget {
            super::app::Widget::QuestionList(ql) => {
                ql.previous();
                app.update_question_in_popup()?;
            }
            super::app::Widget::TopicTagList(tt) => tt.previous(),
        },
        KeyCode::Down => match curr_widget {
            super::app::Widget::QuestionList(ql) => {
                ql.next();
                app.update_question_in_popup()?;
            }
            super::app::Widget::TopicTagList(tt) => tt.next(),
        },
        KeyCode::Left => app.prev_widget(),
        KeyCode::Right => app.next_widget(),
        // Other handlers you could add here.
        _ => {}
    }
    app.update_question_list();

    // post key event update the question list
    Ok(())
}
