use std::hash::Hash;

use crossterm::event::{KeyCode, ModifierKeyCode};

#[derive(Debug, Clone)]
pub struct HelpText {
    button: Vec<KeyCode>,
    title: String,
}

impl Eq for HelpText {}

impl PartialEq for HelpText {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

/// char('s') -> solve, char('s') -> show_solution  is not possible.
/// Hence hashing only button values so that multiple actions cannot point to single key
impl Hash for HelpText {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.button.hash(state);
    }
}

impl HelpText {
    pub fn get_keys(&self) -> std::slice::Iter<KeyCode> {
        self.button.iter()
    }
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
            .map(HelpText::get_symbol_by_keycode)
            .collect::<String>();
        format!("{}[{}]", value.title, symbols,)
    }
}

pub(crate) enum CommonHelpText {
    ScrollUp,
    ScrollDown,
    SwitchPane,
    Edit,
    ReadContent,
    // Submit,
    Run,
    Close,
    Select,
}

impl From<CommonHelpText> for HelpText {
    fn from(value: CommonHelpText) -> Self {
        let (k, t) = match value {
            CommonHelpText::ScrollUp => (vec![KeyCode::Up], "Up"),
            CommonHelpText::ScrollDown => (vec![KeyCode::Down], "Down"),
            CommonHelpText::SwitchPane => (vec![KeyCode::Left, KeyCode::Right], "Switch Pane"),
            CommonHelpText::Edit => (vec![KeyCode::Char('E'), KeyCode::Char('e')], "Edit"),
            CommonHelpText::ReadContent => (vec![KeyCode::Enter], "Read Content"),
            CommonHelpText::Close => (vec![KeyCode::Esc], "Close"),
            CommonHelpText::Select => (vec![KeyCode::Enter], "Select"),
            // CommonHelpText::Submit => (
            //     vec![
            //         KeyCode::Modifier(ModifierKeyCode::LeftControl),
            //         KeyCode::Enter,
            //     ],
            //     "Submit",
            // ),
            CommonHelpText::Run => (vec![KeyCode::Char('R'), KeyCode::Char('r')], "Run"),
        };
        HelpText {
            button: k,
            title: t.to_string(),
        }
    }
}
