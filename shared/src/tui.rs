use crossterm::terminal::WindowSize;
use crossterm::{
    cursor,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::mem;
use std::{
    io::Stdout,
    ops::{Deref, DerefMut},
};

use color_eyre::eyre::Result;
use ratatui::{prelude::CrosstermBackend as Backend, Terminal};

pub type Frame<'a> = ratatui::Frame<'a, Backend<std::io::Stdout>>;

pub struct Term {
    pub terminal: Terminal<Backend<Stdout>>,
}

impl Term {
    pub fn start() -> Result<Self> {
        let terminal = Terminal::new(Backend::new(std::io::stdout()))?;
        let mut term = Self { terminal };
        term.enter()?;
        Ok(term)
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.flush()?;
            crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    pub fn suspend(&mut self) -> Result<()> {
        self.exit()?;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        self.enter()?;
        let size = self.size()?;
        self.resize(size)?;
        Ok(())
    }

    pub fn size() -> WindowSize {
        let mut size = WindowSize {
            rows: 0,
            columns: 0,
            width: 0,
            height: 0,
        };
        if let Ok(s) = crossterm::terminal::window_size() {
            let _ = mem::replace(&mut size, s);
        }

        if size.rows == 0 || size.columns == 0 {
            if let Ok(s) = crossterm::terminal::size() {
                size.columns = s.0;
                size.rows = s.1;
            }
        }

        // TODO: Use `CSI 14 t` to get the actual size of the terminal
        // if size.width == 0 || size.height == 0 {}

        size
    }
}

impl Deref for Term {
    type Target = ratatui::Terminal<Backend<std::io::Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
