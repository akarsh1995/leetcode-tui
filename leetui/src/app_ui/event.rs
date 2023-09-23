use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use futures::StreamExt;

use crossterm::event::EventStream;

use crate::errors::AppResult;

/// Terminal events.
#[derive(Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),

    TaskResponse(Box<TaskResponse>),
    /// redraws the terminal
    Redraw,
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

pub use tokio::sync::mpsc::channel as vim_ping_channel;

use super::async_task_channel::TaskResponse;
pub type VimPingSender = tokio::sync::mpsc::Sender<i32>;
pub type VimPingReceiver = tokio::sync::mpsc::Receiver<i32>;

// should be in the main thread to funtion
pub async fn look_for_events(
    tick_rate: u64,
    sender: std::sync::mpsc::Sender<Event>,
    vim_running_loop_ref: Arc<AtomicBool>,
    mut vim_rx: VimPingReceiver,
    mut should_stop_looking_events: tokio::sync::oneshot::Receiver<bool>,
) -> AppResult<()> {
    let mut tick_rate = tokio::time::interval(Duration::from_millis(tick_rate));

    let mut reader = EventStream::new();

    loop {
        tokio::select! {
            _ = tick_rate.tick() => {
                sender.send(Event::Tick)?
            },
            maybe_stop = &mut should_stop_looking_events => {
                if let Ok(stop) = maybe_stop {
                    if stop {
                        break;
                    }
                }
            }
            maybe_event = reader.next() => {
                match maybe_event {
                    Some(event) => {
                        match event? {
                            CrosstermEvent::Key(e) => {
                                if vim_running_loop_ref.load(std::sync::atomic::Ordering::Relaxed) {
                                    vim_rx.recv().await.unwrap();
                                    sender.send(Event::Redraw)?
                                } else {
                                    sender.send(Event::Key(e))?
                                }
                            },
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
