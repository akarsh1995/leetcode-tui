use core::{emit, Event};

use color_eyre::Result;
use config::{constants::EDITOR, DB_CLIENT};
use crossterm::event::KeyEvent;
use leetcode_db::{DbQuestion, DbTopic};
use shared::tui::Term;

use crate::{ctx::Ctx, executor::Executor, root::Root, signals::Signals};

pub struct App {
    cx: super::ctx::Ctx,
    term: Option<Term>,
    signals: Signals,
}

impl App {
    pub async fn run() -> Result<()> {
        let term = Term::start()?;
        let signals = Signals::start()?;
        let mut app = Self {
            cx: Ctx::new(DB_CLIENT.as_ref()).await,
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
                Event::Topic(topic) => app.dispatch_topic_update(topic),
                Event::Questions(qs) => app.dispatch_question_update(qs),
                Event::Popup(lines) => app.dispatch_popup(lines),
                Event::SelectPopup(lines, result_sender) => app.dispatch_select_popup(lines, result_sender),
                Event::Error(e) => app.dispatch_popup(vec![e]),
                Event::Open(file_path) => app.dispatch_opener(file_path),
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
        self.cx.question.get_questions_by_topic(topic)
    }

    fn dispatch_question_update(&mut self, questions: Vec<DbQuestion>) {
        self.cx.question.set_questions(questions);
        emit!(Render);
    }

    fn dispatch_render(&mut self) {
        if let Some(term) = &mut self.term {
            let _ = term.draw(|f| {
                f.render_widget(Root::new(&mut self.cx), f.size());
            });
        }
    }

    fn dispatch_popup(&mut self, lines: Vec<String>) {
        self.cx.popup.set_lines(lines);
        self.cx.popup.toggle();
        emit!(Render);
    }

    fn dispatch_select_popup(
        &mut self,
        lines: Vec<String>,
        sender: tokio::sync::oneshot::Sender<Option<usize>>,
    ) {
        self.cx.select_popup.with_items(lines, sender);
        self.cx.select_popup.toggle();
        emit!(Render);
    }

    fn dispatch_opener(&mut self, file_path: std::path::PathBuf) {
        // TODO: unwraps handling
        if let Some(term) = &mut self.term {
            term.suspend().unwrap();
            let editor = EDITOR.get().expect("editor not set");
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    "{} {}",
                    editor,
                    file_path.as_os_str().to_str().unwrap()
                ))
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            term.resume().unwrap();
            emit!(Render);
        }
    }
}
