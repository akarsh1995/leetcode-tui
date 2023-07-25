pub mod paragraph;

use std::io::Stderr;

use crossterm::event::KeyEvent;
use indexmap::IndexSet;
use ratatui::prelude::*;

use super::help_text::HelpText;

pub type CrossTermStderr = CrosstermBackend<Stderr>;
pub type TermBackend = Terminal<CrossTermStderr>;
pub type FrameBackend<'a> = Frame<'a, CrossTermStderr>;

pub(crate) trait Component {
    fn event_handler(&mut self, event: KeyEvent) -> Option<KeyEvent>;
    fn render(&self, f: &mut Frame<CrosstermBackend<Stderr>>, render_area: Rect);
    fn get_common_state_mut(&mut self) -> &mut CommonState;
    fn get_common_state(&self) -> &CommonState;
    fn get_key_set(&self) -> IndexSet<HelpText> {
        self.get_common_state().help_text.clone()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CommonState {
    help_text: IndexSet<HelpText>,
    title: String,
}
