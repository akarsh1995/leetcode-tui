use super::app::App;
use crate::errors::AppResult;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // if has active popups then send events to popup
    if app.has_popups() {
        if let Some(last_popup) = app.popups.last_mut() {
            last_popup.handler(key_event)?;
            return Ok(());
        }
    }

    match key_event.code {
        KeyCode::Left => app.next_widget(),
        KeyCode::Right => app.prev_widget(),
        KeyCode::Char('q') | KeyCode::Char('Q') => app.running = false,
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        _ => app.get_current_widget_mut().handler(key_event)?,
    }
    Ok(())
}
