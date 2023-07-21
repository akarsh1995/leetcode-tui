use crossterm::event::{KeyCode, ModifierKeyCode};

#[derive(Debug, Clone)]
pub struct HelpText {
    button: Vec<KeyCode>,
    title: String,
}

impl HelpText {
    pub fn new(title: String, button: Vec<KeyCode>) -> Self {
        Self { button, title }
    }
    fn get_symbol_by_keycode(k: &KeyCode) -> String {
        match k {
            KeyCode::Backspace => "⌫ ".to_string(),
            KeyCode::Enter => "⏎".to_string(),
            KeyCode::Left => "←".to_string(),
            KeyCode::Right => "→".to_string(),
            KeyCode::Up => "↑".to_string(),
            KeyCode::Down => "↓".to_string(),
            KeyCode::Home => "⤒".to_string(),
            KeyCode::End => "⤓".to_string(),
            KeyCode::PageUp => "⇞".to_string(),
            KeyCode::PageDown => "⇟".to_string(),
            KeyCode::Tab => "⇥".to_string(),
            KeyCode::BackTab => "⇤".to_string(),
            KeyCode::Delete => "⌦ ".to_string(),
            KeyCode::Insert => "⎀".to_string(),
            KeyCode::F(n) => format!("F{}", n),
            KeyCode::Char(c) => c.to_string(),
            KeyCode::Null => "∅".to_string(),
            KeyCode::Esc => "⎋".to_string(),
            KeyCode::CapsLock => "⇪".to_string(),
            KeyCode::ScrollLock => "⤓".to_string(),
            KeyCode::NumLock => "⇭".to_string(),
            KeyCode::PrintScreen => "⎙".to_string(),
            KeyCode::Pause => "⏸".to_string(),
            KeyCode::Menu => "☰".to_string(),
            KeyCode::KeypadBegin => "⎆".to_string(),
            KeyCode::Media(_) => "☊".to_string(),
            KeyCode::Modifier(modifier) => match modifier {
                ModifierKeyCode::LeftShift => "⇧".to_string(),
                ModifierKeyCode::LeftControl => "⌃".to_string(),
                ModifierKeyCode::LeftAlt => "⌥".to_string(),
                ModifierKeyCode::LeftSuper => "⌘".to_string(),
                ModifierKeyCode::LeftHyper => "⎇".to_string(),
                ModifierKeyCode::LeftMeta => "⌘".to_string(),
                ModifierKeyCode::RightShift => "⇧".to_string(),
                ModifierKeyCode::RightControl => "⌃".to_string(),
                ModifierKeyCode::RightAlt => "⌥".to_string(),
                ModifierKeyCode::RightSuper => "⌘".to_string(),
                ModifierKeyCode::RightHyper => "⎇".to_string(),
                ModifierKeyCode::RightMeta => "⌘".to_string(),
                ModifierKeyCode::IsoLevel3Shift => "ISO Level3 Shift".to_string(),
                ModifierKeyCode::IsoLevel5Shift => "ISO Level5 Shift".to_string(),
            },
        }
    }
}

impl From<&HelpText> for String {
    fn from(value: &HelpText) -> Self {
        let symbols = value
            .button
            .iter()
            .map(|k| HelpText::get_symbol_by_keycode(k))
            .collect::<String>();
        format!("{}{}", symbols, value.title)
    }
}