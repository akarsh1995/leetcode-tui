use core::Event;

use color_eyre::Result;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind};
use futures::StreamExt;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

pub(super) struct Signals {
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,
}

impl Signals {
    pub(super) fn start() -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut signals = Self { tx: tx.clone(), rx };

        Event::init(tx);
        signals.spawn_crossterm_task();
        Ok(signals)
    }

    pub(super) async fn recv(&mut self) -> Option<Event> {
        self.rx.recv().await
    }

    fn spawn_crossterm_task(&mut self) -> JoinHandle<()> {
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let mut reader = EventStream::new();

            loop {
                tokio::select! {
                    Some(Ok(event)) = reader.next() => {
                        let event = match event {
                            // We need to check key event kind;
                            // otherwise event will be dispatched twice.
                            CrosstermEvent::Key(key @ KeyEvent { kind: KeyEventKind::Press, .. }) => {
                                let k: config::key::Key = key.into();
                                if let config::key::Key::Ctrl('c') = k {
                                    Event::Quit
                                } else {
                                    Event::Key(key)
                                }
                            },
                            // CrosstermEvent::Paste(str) => Event::Paste(str),
                            CrosstermEvent::Resize(cols, rows) => Event::Resize(cols, rows),
                            _ => continue,
                        };
                        if tx.send(event).is_err() {
                            break;
                        }
                    }
                }
            }
        })
    }
}
