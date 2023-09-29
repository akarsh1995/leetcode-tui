use core::{emit, Event};

use color_eyre::Result;
use crossterm::event::KeyEvent;
use leetcode_db::Db;
use shared::tui::Term;

use crate::{ctx::Ctx, executor::Executor, root::Root, signals::Signals};

pub struct App {
    cx: super::ctx::Ctx,
    term: Option<Term>,
    signals: Signals,
}

impl App {
    pub async fn run(db: &Db) -> Result<()> {
        let term = Term::start()?;
        let signals = Signals::start()?;
        let mut app = Self {
            cx: Ctx::new(db).await,
            term: Some(term),
            signals,
        };
        emit!(Render);
        while let Some(event) = app.signals.recv().await {
            match event {
                Event::Quit => {
                    // app.dispatch_quit();
                    break;
                }
                Event::Key(key) => app.dispatch_key(key),
                Event::Render(_) => app.dispatch_render(),
                _ => {}
                // Event::Paste(str) => app.dispatch_paste(str),
                // Event::Resize(..) => app.dispatch_resize(),
                // Event::Stop(state, tx) => app.dispatch_stop(state, tx),
                // Event::Call(exec, layer) => app.dispatch_call(exec, layer),
                // event => app.dispatch_module(event),
            }
        }
        Ok(())
    }

    fn dispatch_key(&mut self, key: KeyEvent) {
        let key = config::key::Key::from(key);
        if Executor::handle(&mut self.cx, key) {
            emit!(Render);
        }
    }

    fn dispatch_render(&mut self) {
        if let Some(term) = &mut self.term {
            let _ = term.draw(|f| {
                f.render_widget(Root::new(&self.cx), f.size());
            });
        }
    }
}
