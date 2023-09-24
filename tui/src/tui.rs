use crossterm::{
    cursor,
    event::{Event as CrosstermEvent, MouseEvent},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::key::Key;
use color_eyre::eyre::Result;
use ratatui::{prelude::CrosstermBackend as Backend, Terminal};
use std::io::Stderr;

pub type Frame<'a> = ratatui::Frame<'a, Backend<std::io::Stderr>>;

#[derive(Debug, Clone)]
pub enum Event {
    Init,
    Render,
    Tick,
    Key(Key),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Redraw,
    Quit,
}

pub struct Tui {
    pub terminal: Terminal<Backend<Stderr>>,
    pub event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    pub event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    pub cancellation_token: CancellationToken,
    pub frame_rate: f64,
    pub tick_rate: f64,
    pub task: JoinHandle<Result<(), tokio::sync::mpsc::error::SendError<Event>>>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let tick_rate = 4.0;
        let frame_rate = 60.0;
        let terminal = Terminal::new(Backend::new(std::io::stderr()))?;
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let task = tokio::spawn(async move { Ok(()) });
        let cancellation_token = CancellationToken::new();
        Ok(Self {
            terminal,
            event_tx,
            event_rx,
            tick_rate,
            frame_rate,
            task,
            cancellation_token,
        })
    }

    pub fn tick_rate(&mut self, tick_rate: f64) {
        self.tick_rate = tick_rate;
    }

    pub fn frame_rate(&mut self, frame_rate: f64) {
        self.frame_rate = frame_rate;
    }

    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;
        self.start();
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.cancel();
        let mut counter = 0;
        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));
            counter += 1;
            if counter > 50 {
                self.task.abort();
            }
            if counter > 100 {
                log::error!("Failed to abort task in 100 milliseconds for unknown reason");
                break;
            }
        }
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.stop()?;
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.flush()?;
            crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    pub fn suspend(&mut self) -> Result<()> {
        self.exit()?;
        #[cfg(not(windows))]
        signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        self.enter()?;
        Ok(())
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }

    pub fn start(&mut self) {
        let tick_delay = Duration::from_secs_f64(1.0 / self.tick_rate);
        let render_delay = Duration::from_secs_f64(1.0 / self.frame_rate);
        let _event_tx = self.event_tx.clone();

        // cancellation token
        self.cancel();
        self.cancellation_token = CancellationToken::new();
        let _cancellation_token = self.cancellation_token.clone();

        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);

            _event_tx.send(Event::Init)?;
            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next();

                tokio::select! {
                    _ = _cancellation_token.cancelled() => {
                        break;
                    }
                    _ = tick_delay => {
                        _event_tx.send(Event::Tick)?
                    },
                    _ = render_delay => {
                        _event_tx.send(Event::Render)?
                    },
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(event) => {
                                match event.unwrap() {
                                    CrosstermEvent::Key(e) => {
                                        _event_tx.send(
                                            Event::Key(Key::from(e))
                                        )?
                                    },
                                    CrosstermEvent::Mouse(e) => _event_tx.send(
                                        Event::Mouse(e)
                                    )?,
                                    CrosstermEvent::Resize(w, h) => _event_tx.send(
                                        Event::Resize(w, h)
                                    )?,
                                    _ => unimplemented!()
                                }
                            }
                            None => break,
                        }
                    }
                }
            }
            Ok(())
        });
    }
}

impl Deref for Tui {
    type Target = ratatui::Terminal<Backend<std::io::Stderr>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
