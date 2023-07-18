use super::app::AppResult;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
// use std::sync::mpsc;
use std::time::Duration;

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::event::EventStream;

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    pub sender: crossbeam::channel::Sender<Event>,
    /// Event receiver channel.
    pub receiver: crossbeam::channel::Receiver<Event>,
    // /// Event handler thread.
    // handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub async fn new(tick_rate: u64, sender: crossbeam::channel::Sender<Event>) {
        let tick_rate = Duration::from_millis(tick_rate);

        let mut reader = EventStream::new();

        loop {
            let mut delay = Delay::new(tick_rate).fuse();
            let mut event = reader.next().fuse();

            select! {
                _ = delay => {
                    sender.send(Event::Tick).expect("Some Error")
                },
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            match event {
                                CrosstermEvent::Key(e) => sender.send(Event::Key(e)).unwrap(),
                                CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)).unwrap(),
                                CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)).unwrap(),
                                _ => unimplemented!()
                            }
                        }
                        Some(Err(e)) => {println!("Error: {:?}\r", e);},
                        None => break,
                    }
                }
            }
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}
