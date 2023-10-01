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
    Popup(Vec<String>),
}

impl Event {
    #[inline]
    pub fn init(tx: UnboundedSender<Event>) {
        TX.init(tx);
    }

    #[inline]
    pub fn emit(self) {
        TX.send(self).ok();
    }

    pub async fn wait<T>(self, rx: oneshot::Receiver<T>) -> T {
        TX.send(self).ok();
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
    ($event:ident) => {
        $crate::Event::$event.emit();
    };
}
