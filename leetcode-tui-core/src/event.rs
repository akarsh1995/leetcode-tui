use crossterm::event::KeyEvent;
use leetcode_tui_db::{DbQuestion, DbTopic};
use leetcode_tui_shared::RoCell;
use std::path::PathBuf;

use tokio::sync::{mpsc::UnboundedSender, oneshot};

static TX: RoCell<UnboundedSender<Event>> = RoCell::new();

pub enum Event {
    Quit,
    Key(KeyEvent),
    Render(String),
    Resume,
    Suspend,
    Resize(u16, u16),
    Topic(DbTopic),
    Questions(Vec<DbQuestion>),
    AddQuestions(Vec<DbQuestion>),
    AdhocQuestion(DbQuestion),
    QuestionFilter(Option<String>),
    Popup(Option<String>, Vec<String>),
    SelectPopup(
        Option<String>,
        Vec<String>,
        tokio::sync::oneshot::Sender<Option<usize>>,
    ),
    Input(super::UBStrSender, Option<String>),
    Open(PathBuf),
    Error(String),
    QuestionUpdate,
    SyncDb,
    ProgressUpdate(String, u32, u32),
}

impl Event {
    #[inline]
    pub fn init(tx: UnboundedSender<Event>) {
        TX.init(tx);
    }

    #[inline]
    pub fn emit(self) {
        TX.as_ref().send(self).ok();
    }

    pub async fn wait<T>(self, rx: oneshot::Receiver<T>) -> T {
        TX.as_ref().send(self).ok();
        rx.await.unwrap_or_else(|_| std::process::exit(0))
    }
}

#[macro_export]
macro_rules! emit {
    (Key($key:expr)) => {
        $crate::Event::Key($key).emit();
    };
    (Render) => {
        $crate::Event::Render(format!("{}:{}", file!(), line!())).emit();
    };
    (Resize($cols:expr, $rows:expr)) => {
        $crate::Event::Resize($cols, $rows).emit();
    };
    (Topic($topic:expr)) => {
        $crate::Event::Topic($topic).emit();
    };
    (Questions($questions:expr)) => {
        $crate::Event::Questions($questions).emit();
    };
    (AddQuestions($questionList:expr)) => {
        $crate::Event::AddQuestions($questionList).emit();
    };
    (AdhocQuestion($question:expr)) => {
        $crate::Event::AdhocQuestion($question).emit();
    };
    (Popup($lines:expr)) => {
        $crate::Event::Popup(None, $lines).emit();
    };
    (Popup($title:expr, $lines:expr)) => {
        $crate::Event::Popup(Some($title.into()), $lines).emit();
    };
    (SelectPopup($a: expr)) => {{
        let (tx, rx) = tokio::sync::oneshot::channel();
        $crate::Event::SelectPopup(None, $a, tx).wait(rx)
    }};
    (SelectPopup($title:expr, $a: expr)) => {{
        let (tx, rx) = tokio::sync::oneshot::channel();
        $crate::Event::SelectPopup(Some($title.into()), $a, tx).wait(rx)
    }};
    (Error($e:expr)) => {
        $crate::Event::Error($e).emit();
    };
    (Open($e:expr)) => {
        $crate::Event::Open($e).emit();
    };
    (Input($e:expr)) => {{
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        $crate::Event::Input(tx, $e).emit();
        rx
    }};
    (QuestionFilter($e:expr)) => {
        $crate::Event::QuestionFilter($e).emit();
    };
    (ProgressUpdate($title:expr, $progress:expr, $total:expr)) => {
        $crate::Event::ProgressUpdate($title, $progress, $total).emit();
    };
    ($event:ident) => {
        $crate::Event::$event.emit();
    };
}
