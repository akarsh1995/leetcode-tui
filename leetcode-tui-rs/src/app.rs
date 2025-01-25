use crate::utils::update_database_questions;
use crate::{ctx::Ctx, executor::Executor, signals::Signals, widgets::root::Root};
use color_eyre::Result;
use leetcode_tui_config::{constants::EDITOR, key::Key};
use leetcode_tui_core::{emit, Event, UBStrSender};
use leetcode_tui_db::{DbQuestion, DbTopic};
use leetcode_tui_shared::tui::Term;

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
            cx: Ctx::new().await,
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
                Event::Input(sender, default_input) => app.dispatch_input(sender, default_input),
                Event::Key(key) => app.dispatch_key(key),
                Event::Render(_) => app.dispatch_render(),
                Event::Topic(topic) => app.dispatch_topic_update(topic),
                Event::AddQuestions(qs) => app.dispatch_add_question(qs),
                Event::AdhocQuestion(qs) => app.dispatch_adhoc_question(qs),
                Event::Questions(qs) => app.dispatch_question_update(qs),
                Event::Popup(title, lines) => app.dispatch_popup(title, lines),
                Event::SelectPopup(maybe_title, lines, result_sender) => {
                    app.dispatch_select_popup(maybe_title, lines, result_sender)
                }
                Event::Error(e) => app.dispatch_popup(Some("Error".into()), vec![e]),
                Event::Open(file_path) => app.dispatch_opener(file_path),
                Event::SyncDb => app.dispatch_db_update().await,
                Event::ProgressUpdate(title, progress, total) => {
                    app.dispatch_progress_update(title, progress, total)
                }
                e => app.dispatch_module_event(e),
                // Event::Paste(str) => app.dispatch_paste(str),
                // Event::Resize(..) => app.dispatch_resize(),
                // Event::Stop(state, tx) => app.dispatch_stop(state, tx),
                // Event::Call(exec, layer) => app.dispatch_call(exec, layer),
                // event => app.dispatch_module(event),
            }
        }
        Ok(())
    }

    fn dispatch_key(&mut self, key: impl Into<Key>) {
        if Executor::handle(&mut self.cx, key.into()) {
            emit!(Render);
        }
    }

    fn dispatch_topic_update(&mut self, topic: DbTopic) {
        self.cx
            .content
            .get_questions_mut()
            .get_questions_by_topic(topic)
    }

    fn dispatch_question_update(&mut self, questions: Vec<DbQuestion>) {
        self.cx.content.get_questions_mut().set_questions(questions);
        emit!(Render);
    }

    fn dispatch_add_question(&mut self, questions: Vec<DbQuestion>) {
        for ques in questions {
            self.cx.content.get_questions_mut().add_question(ques);
        }
    }

    fn dispatch_render(&mut self) {
        if let Some(term) = &mut self.term {
            let _ = term.draw(|f| {
                f.render_widget(Root::new(&mut self.cx), f.size());
            });
        }
    }

    fn dispatch_popup(&mut self, title: Option<String>, lines: Vec<String>) {
        self.cx.popup.reset(title, lines);
        self.cx.popup.toggle();
        emit!(Render);
    }

    fn dispatch_select_popup(
        &mut self,
        maybe_title: Option<String>,
        lines: Vec<String>,
        sender: tokio::sync::oneshot::Sender<Option<usize>>,
    ) {
        self.cx.select_popup.with_items(maybe_title, lines, sender);
        self.cx.select_popup.toggle();
        emit!(Render);
    }

    fn dispatch_opener(&mut self, file_path: std::path::PathBuf) {
        // TODO: unwraps handling
        self.signals.stop_looking_for_io_events();
        if let Some(term) = &mut self.term {
            term.suspend().unwrap();
            let editor = EDITOR.get().expect("editor not set");
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    r#"{} "{}""#,
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
        self.signals.start_looking_for_io_events();
    }

    fn dispatch_module_event(&mut self, e: Event) {
        match e {
            Event::QuestionFilter(needle) => self.cx.content.get_questions_mut().filter_by(needle),
            Event::QuestionUpdate => self.cx.content.get_topic().notify_change(),
            _ => (),
        }
        emit!(Render);
    }

    fn dispatch_input(&mut self, sender: UBStrSender, default_input: Option<String>) {
        self.cx.input.toggle();
        self.cx.input.reset_with(sender, default_input);
        emit!(Render);
    }

    fn dispatch_adhoc_question(&mut self, qs: DbQuestion) {
        self.cx.content.get_questions_mut().set_adhoc(qs);
        self.cx.content.get_questions_mut().show_question_content();
    }

    async fn dispatch_db_update(&mut self) {
        tokio::spawn(async move {
            update_database_questions(true).await.unwrap();
        });
    }

    fn dispatch_progress_update(&mut self, title: String, progress: u32, total: u32) {
        self.cx.progress.set_progress(title, progress, total);
        emit!(Render);
    }
}
