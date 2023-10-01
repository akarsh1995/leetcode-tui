use core::{emit, Event};

use color_eyre::Result;
use crossterm::event::KeyEvent;
use leetcode_db::{Db, DbQuestion, DbTopic};
use shared::tui::Term;

use crate::{ctx::Ctx, executor::Executor, root::Root, signals::Signals};

pub struct App {
    cx: super::ctx::Ctx,
    term: Option<Term>,
    signals: Signals,
    db: Db,
}

impl App {
    pub async fn run(db: &Db) -> Result<()> {
        let term = Term::start()?;
        let signals = Signals::start()?;
        let mut app = Self {
            cx: Ctx::new(db).await,
            term: Some(term),
            signals,
            db: db.clone(),
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
                Event::Topic(topic) => app.dispatch_topic_update(topic),
                Event::Questions(qs) => app.dispatch_question_update(qs),
                Event::Popup(lines) => app.dispatch_popup(lines),
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

    fn dispatch_topic_update(&mut self, topic: DbTopic) {
        self.cx
            .question
            .get_questions_by_topic(topic, self.db.clone())
    }

    fn dispatch_question_update(&mut self, questions: Vec<DbQuestion>) {
        self.cx.question.set_questions(questions)
    }

    fn dispatch_render(&mut self) {
        if let Some(term) = &mut self.term {
            let _ = term.draw(|f| {
                f.render_widget(Root::new(&self.cx), f.size());
            });
        }
    }

    fn dispatch_popup(&mut self, lines: Vec<String>) {
        self.cx.popup.set_lines(lines);
        self.cx.popup.toggle();
    }
}
