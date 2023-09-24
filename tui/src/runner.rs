use color_eyre::eyre::Result;
use leetcode_db::Db;
use ratatui::prelude::Layout;
use ratatui::prelude::*;
use tokio::sync::mpsc;

use crate::{
    action::Action,
    components::{help_bar::Help, question::Question, topic::TopicComp, Component},
    key::Key,
    // config::Config,
    tui::{self, Frame, Tui},
};

pub struct Runner {
    // pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
}

impl Runner {
    pub async fn new(tick_rate: f64, frame_rate: f64, db: &Db) -> Result<Self> {
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![
                Box::new(TopicComp::new(db).await),
                Box::new(Question::new()),
                Box::new(Help::new()),
            ],
            should_quit: false,
            should_suspend: false,
            // config,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = Tui::new()?;
        tui.tick_rate(self.tick_rate);
        tui.frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
            component.register_event_handler(tui.event_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init()?;
        }

        let vertical_chunks = |f: &mut Frame<'_>| {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
                .split(f.size())
        };

        let horizontal_chunks = |f: &mut Frame<'_>| {
            let chunks = vertical_chunks(f);
            let top_section = chunks[0];
            let bottom_section = chunks[1];
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(top_section);
            let top_left = chunks[0];
            let top_right = chunks[1];
            [top_left, top_right, bottom_section]
        };

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    // Quite here means control c
                    tui::Event::Key(Key::Char('q') | Key::Ctrl('c')) => {
                        action_tx.send(Action::Quit)?
                    }
                    tui::Event::Tick => action_tx.send(Action::Tick)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Init => action_tx.send(Action::Init)?,
                    e => {
                        for component in self.components.iter_mut() {
                            if let Some(action) = component.handle_events(Some(e.clone()))? {
                                action_tx.send(action)?;
                            }
                        }
                    }
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                if !matches!(action, Action::Tick) && !matches!(action, Action::Render) {
                    log::debug!("{action:?}");
                }

                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }

                match action {
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::Init => action_tx.send(Action::SetHelpBar(vec![
                        (
                            vec![Key::Char('t'), Key::Char('T')],
                            "Change Topic".to_string(),
                        ),
                        (vec![Key::Down, Key::Up], "Select Question".into()),
                    ]))?,
                    Action::Render | Action::Resize(_, _) => {
                        tui.draw(|f| {
                            let sections = horizontal_chunks(f);
                            for (i, component) in self.components.iter_mut().enumerate() {
                                let r = component.draw(f, sections[i]);
                                action_tx
                                    .send(Action::Error(format!("Failed to draw {:?}", r)))
                                    .unwrap();
                            }
                        })?;
                    }
                    _ => {}
                }
            }

            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                tui = Tui::new()?;
                tui.tick_rate(self.tick_rate);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }
}
