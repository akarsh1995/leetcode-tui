pub mod help_bar;
pub mod question;
pub mod topic;

use color_eyre::eyre::Result;
use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    key::Key,
    tui::{Event, Frame},
};

//// ANCHOR: component
pub trait Component {
    #[allow(unused_variables)]
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_event_handler(&mut self, tx: UnboundedSender<Event>) -> Result<()> {
        Ok(())
    }

    fn init(&mut self) -> Result<()> {
        Ok(())
    }
    fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
        let r = match event {
            Some(Event::Key(key_event)) => self.handle_key_event(key_event)?,
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event)?,
            _ => None,
        };
        Ok(r)
    }

    #[allow(unused_variables)]
    fn handle_key_event(&mut self, key: Key) -> Result<Option<Action>> {
        Ok(None)
    }

    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
        Ok(None)
    }

    #[allow(unused_variables)]
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()>;
}
//// ANCHOR_END: component
