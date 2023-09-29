use std::fmt::Display;

use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{action::Action, key::Key, tui::Event};

pub struct Help {
    pub action_tx: Option<UnboundedSender<Action>>,
    pub keys: Vec<(Vec<Key>, String)>,
    pub event_tx: Option<UnboundedSender<Event>>,
}

impl Help {
    pub fn new() -> Self {
        Self {
            action_tx: Default::default(),
            event_tx: None,
            keys: vec![],
        }
    }
}

impl Component for Help {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn register_event_handler(&mut self, tx: UnboundedSender<Event>) -> Result<()> {
        self.event_tx = Some(tx);
        Ok(())
    }

    fn handle_key_event(&mut self, _key: Key) -> Result<Option<Action>> {
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        Ok(match action {
            Action::SetHelpBar(keys) => {
                self.keys = keys;
                Some(Action::Render)
            }
            _ => None,
        })
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let help_texts = &self.keys;
        let spans = help_texts
            .into_iter()
            .flat_map(|(keys, string)| {
                vec![
                    Span::from(format!(
                        "[{}] {}",
                        keys.iter()
                            .map(|key| key.to_string())
                            .collect::<Vec<_>>()
                            .join("/"),
                        string
                    ))
                    .bg(Color::Cyan)
                    .fg(Color::White),
                    Span::from(" "),
                ]
            })
            .collect::<Vec<_>>();
        let line = Line::from(spans).alignment(Alignment::Right);
        let p = Paragraph::new(line); //.block(block);
        f.render_widget(p, rect);
        Ok(())
    }
}

// impl Display for &(Vec<Key>, String) {}
