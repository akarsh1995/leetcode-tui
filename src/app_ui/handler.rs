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
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            } else if let Widget::QuestionList(s) = app.get_current_widget() {
                if let Some(selected_item) = s.get_selected_item() {
                    if let Some(slug) = &selected_item.title_slug {
                        app.task_request_sender
                            .send(channel::Request::QuestionDetail { slug: slug.clone() })?;
                        app.toggle_popup();
                    }
                }
            }
        }

        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.toggle_popup();
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
