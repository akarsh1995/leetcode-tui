use app_core::Event;

use color_eyre::Result;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind};
use futures::StreamExt;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

pub(super) struct Signals {
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,
    cancellation_token: tokio_util::sync::CancellationToken,
}

impl Signals {
    pub(super) fn start() -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();
        let cancellation_token = tokio_util::sync::CancellationToken::new();
        let mut signals = Self {
            tx: tx.clone(),
            rx,
            cancellation_token,
        };

        Event::init(tx);
        signals.spawn_crossterm_task();
        Ok(signals)
    }

    pub(super) async fn recv(&mut self) -> Option<Event> {
        self.rx.recv().await
    }

    pub fn stop_looking_for_io_events(&mut self) {
        self.cancellation_token.cancel()
    }

    pub fn start_looking_for_io_events(&mut self) {
        self.reset_cancellation_token();
        self.spawn_crossterm_task();
    }

    fn reset_cancellation_token(&mut self) {
        self.cancellation_token = CancellationToken::new();
    }

    fn spawn_crossterm_task(&mut self) -> JoinHandle<()> {
        let tx = self.tx.clone();
        let token = self.cancellation_token.clone();

        tokio::spawn(async move {
            let mut reader = EventStream::new();

            loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        break;
                    }

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
