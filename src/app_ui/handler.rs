use super::{
    app::App,
    widgets::{notification::Notification, Widget},
};
use crate::errors::AppResult;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<Option<Notification>> {
    // if ui has active popups then send only events registered with popup
    if let Some(popup) = app.get_current_popup_mut() {
        return popup.handler(key_event);
    }

    match key_event.code {
        KeyCode::Left => return app.next_widget(),
        KeyCode::Right => return app.prev_widget(),
        KeyCode::Char('q') | KeyCode::Char('Q') => app.running = false,
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        _ => return app.get_current_widget_mut().handler(key_event),
    };

    Ok(None)
}
