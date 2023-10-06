use std::path::PathBuf;

use crossterm::event::KeyEvent;
use leetcode_db::{DbQuestion, DbTopic};
use shared::RoCell;

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
    QuestionFilter(Option<String>),
    Popup(Vec<String>),
    SelectPopup(Vec<String>, tokio::sync::oneshot::Sender<Option<usize>>),
    Input(super::UBStrSender, Option<String>),
    Open(PathBuf),
    Error(String),
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
    (Popup($lines:expr)) => {
        $crate::Event::Popup($lines).emit();
    };
    (SelectPopup($a: expr)) => {{
        let (tx, rx) = tokio::sync::oneshot::channel();
        $crate::Event::SelectPopup($a, tx).wait(rx)
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
    ($event:ident) => {
        $crate::Event::$event.emit();
    };
}
