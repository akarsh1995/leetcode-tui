use std::sync::Arc;

use color_eyre::eyre::Result;
use leetcode_db::{Db, DbTopic as Topic};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{action::Action, key::Key, tui::Event, StatefulList};

pub struct TopicComp {
    pub topics: StatefulList<Topic>,
    pub action_tx: Option<UnboundedSender<Action>>,
    pub db: Db,
    pub event_tx: Option<UnboundedSender<Event>>,
}

impl TopicComp {
    pub async fn new(db: &Db) -> Self {
        let topics = Topic::fetch_all(db).await.unwrap();
        let mut sflist = StatefulList::with_items(Arc::new(topics));
        if sflist.items.len() > 0 {
            sflist.state.select(Some(0));
        }
        Self {
            action_tx: Default::default(),
            topics: sflist,
            db: db.clone(),
            event_tx: None,
        }
    }
}

impl Component for TopicComp {
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
            Key::Char('T') => Some(Action::PreviousTopic),
            Key::Char('t') => Some(Action::NextTopic),
            _ => None,
        })
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        let tx = self.action_tx.clone().unwrap();
        Ok(match action {
            Action::NextTopic => {
                self.topics.next();
                Some(Action::SetQuestions)
            }
            Action::PreviousTopic => {
                self.topics.previous();
                Some(Action::SetQuestions)
            }
            Action::SetQuestions => {
                let selected = self.topics.get_selected_item_ref().unwrap().clone();
                let db = self.db.clone();
                let ev_tx = self.event_tx.clone().unwrap();
                tokio::spawn(async move {
                    let questions = selected.fetch_questions(&db).await;
                    tx.send(Action::UpdateQuestions(Arc::new(questions.unwrap())))
                        .unwrap();
                    ev_tx.send(Event::Render).unwrap();
                });
                None
            }
            Action::Init => Some(Action::SetQuestions),
            _ => None,
        })
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        // let chunks = Layout::default()
        //     .direction(Direction::Horizontal)
        //     .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        //     .split(rect);
        // let chunks = layout.split(rect);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().gray());

        let inner = block.inner(rect);

        f.render_widget(block, rect);

        let items: Vec<ListItem> = self
            .topics
            .items
            .iter()
            .map(|topic| ListItem::new(Line::from(topic.slug.as_str())))
            .collect();

        let items = List::new(items).highlight_style(
            Style::default()
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        );
        f.render_stateful_widget(items, inner, &mut self.topics.state);

        Ok(())
    }
}
