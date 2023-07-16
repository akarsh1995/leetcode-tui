use super::app::{App, AppResult};
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

    // post key event update the question list
    let mut name: Option<String> = None;

    match &app.widgets[app.widget_switcher as usize] {
        super::app::Widget::TopicTagList(ttl) => {
            if let Some(selected_widget) = ttl.get_selected_item() {
                if let Some(n) = &selected_widget.name {
                    name = Some(n.clone());
                }
            }
        }
        _ => {}
    }

    for w in app.widgets.iter_mut() {
        if let super::app::Widget::QuestionList(ql) = w {
            if let Some(name) = &name {
                ql.items = app.questions_list.get(name).unwrap().clone();
            }
        }
    }
    Ok(())
}
