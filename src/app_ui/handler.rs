use super::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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
        KeyCode::Up => { 
            app.topic_tags_stateful.previous();
        }
        KeyCode::Down => {
            app.topic_tags_stateful.next() ;
        }
        // Other handlers you could add here.
        _ => {}
    }

    // post key event update the question list
    if let Some(selected_tt) = app.topic_tags_stateful.get_selected_item() {
        if let Some(ttname) = &selected_tt.name {
            let questions = app.questions_list.get(ttname).unwrap();
            app.questions_stateful.items = questions.clone();
        }
    }
    Ok(())
}
