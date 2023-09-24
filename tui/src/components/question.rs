use color_eyre::eyre::Result;
use leetcode_db::DbQuestion;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{action::Action, key::Key, tui::Event, StatefulList};

pub struct Question {
    pub questions: Option<StatefulList<DbQuestion>>,
    pub action_tx: Option<UnboundedSender<Action>>,
    pub event_tx: Option<UnboundedSender<Event>>,
}

impl Question {
    pub fn new() -> Self {
        Self {
            action_tx: Default::default(),
            questions: None,
            event_tx: None,
        }
    }
}

impl Component for Question {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn register_event_handler(&mut self, tx: UnboundedSender<Event>) -> Result<()> {
        self.event_tx = Some(tx);
        Ok(())
    }

    fn handle_key_event(&mut self, key: Key) -> Result<Option<Action>> {
        Ok(match key {
            Key::Up => Some(Action::PreviousQuestion),
            Key::Down => Some(Action::NextQuestion),
            _ => None,
        })
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        Ok(match action {
            Action::PreviousQuestion => {
                if let Some(_questions) = &mut self.questions {
                    _questions.previous();
                }
                Some(Action::Render)
            }
            Action::NextQuestion => {
                if let Some(_questions) = &mut self.questions {
                    _questions.next();
                }
                Some(Action::Render)
            }
            Action::UpdateQuestions(qs) => {
                let mut question_list = StatefulList::with_items(qs);
                if question_list.items.len() > 0 {
                    question_list.state.select(Some(0));
                }
                self.questions = Some(question_list);
                None
            }
            _ => None,
        })
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().cyan());

        let inner = block.inner(rect);
        f.render_widget(block, rect);

        if let Some(_questions) = &mut self.questions {
            let question_list: Vec<ListItem> = _questions
                .items
                .iter()
                .map(|q| ListItem::new(Line::from(q.title.as_str())))
                .collect();

            let question_list = List::new(question_list).highlight_style(
                Style::default()
                    .bg(Color::LightCyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );
            f.render_stateful_widget(question_list, inner, &mut _questions.state);
        }

        Ok(())
    }
}
