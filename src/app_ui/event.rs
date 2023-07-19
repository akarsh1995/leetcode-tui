use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::time::Duration;

use futures::{future::FutureExt, StreamExt};
use futures_timer::Delay;

use crossterm::event::EventStream;

use crate::errors::AppResult;

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
    pub sender: std::sync::mpsc::Sender<Event>,
    /// Event receiver channel.
    pub receiver: std::sync::mpsc::Receiver<Event>,
}

// should be in the main thread to funtion
pub async fn look_for_events(
    tick_rate: u64,
    sender: std::sync::mpsc::Sender<Event>,
) -> AppResult<()> {
    let tick_rate = Duration::from_millis(tick_rate);

    let mut reader = EventStream::new();

    loop {
        let delay = Delay::new(tick_rate).fuse();
        let event = reader.next().fuse();

        tokio::select! {
            _ = delay => {
                sender.send(Event::Tick)?
            },
            maybe_event = event => {
                match maybe_event {
                    Some(event) => {
                        match event? {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e))?,
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e))?,
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h))?,
                            _ => unimplemented!()
                        }
                    }
                    None => break,
                }
            }
        }
    }
    Ok(())
}

impl EventHandler {
    /// Receive the next event from the handler thread.

    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}
