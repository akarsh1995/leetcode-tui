use super::app::App;
use super::event::EventHandler;
use super::ui;
use crate::errors::{AppResult, LcAppError};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode().map_err(|e| {
            LcAppError::CrossTermError(format!("Error while enabling raw mode. {e}"))
        })?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture).map_err(
            |e| LcAppError::CrossTermError(format!("Error while execute macro execution. {e}")),
        )?;
        self.terminal
            .hide_cursor()
            .map_err(|e| LcAppError::CrossTermError(format!("Error while hiding cursor. {e}")))?;
        self.terminal.clear().map_err(|e| {
            LcAppError::CrossTermError(format!("Error while clearing terminal. {e}"))
        })?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: tui::Terminal::draw
    /// [`rendering`]: crate::ui:render
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal
            .draw(|frame| ui::render(app, frame))
            .map_err(|e| {
                LcAppError::CrossTermError(format!("Error while drawing on terminal frame. {e}"))
            })?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(mut self) -> AppResult<()> {
        terminal::disable_raw_mode().map_err(|e| {
            LcAppError::CrossTermError(format!("Error while disabling raw mode. {e}"))
        })?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture).map_err(
            |e| LcAppError::CrossTermError(format!("Error while execute macro execution. {e}")),
        )?;
        self.terminal
            .show_cursor()
            .map_err(|e| LcAppError::CrossTermError(format!("Error while show cursor. {e}")))?;
        drop(self.events.receiver);
        Ok(())
    }
}
