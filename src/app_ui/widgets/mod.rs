pub(crate) mod notification;
pub(crate) mod popup;
pub mod question_list;
pub mod stats;
pub mod topic_list;

use std::{fmt::Debug, io::Stderr};

use crossterm::event::KeyEvent;
use ratatui::{prelude::Rect, prelude::*, Frame};

use crate::errors::AppResult;

use self::notification::Notification;

use super::channel::TaskResponse;

pub trait StateManager {
    fn set_active(&mut self);
    fn set_inactive(&mut self);
    fn is_active(&self) -> bool;
}

pub trait Widget: Debug + StateManager {
    fn render(&mut self, rect: Rect, frame: &mut Frame<CrosstermBackend<Stderr>>);
    fn handler(&mut self, event: KeyEvent) -> AppResult<()>;

    fn process_task_response(&mut self, response: TaskResponse) -> AppResult<()>;
    fn setup(&mut self) -> AppResult<()> {
        Ok(())
    }
    fn set_response(&mut self);
    fn process_notification(&mut self, notification: &Notification) -> AppResult<()>;
}

pub type WidgetList = Vec<Box<dyn Widget>>;
pub type CrosstermStderr<'a> = Frame<'a, CrosstermBackend<Stderr>>;

pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl From<Colour> for Style {
    /// sets fg color and returns the style
    fn from(val: Colour) -> Self {
        let pair = val;
        let Colour { r, g, b } = pair;
        Style::default().fg(style::Color::Rgb(r, g, b))
    }
}

pub struct Pair {
    pub fg: Colour,
    pub bg: Colour,
}

pub const CHECK_MARK: &str = "✔️";

pub enum Callout {
    Success,
    Info,
    Warning,
    Error,
    Disabled,
}

impl Callout {
    // Method to get the corresponding Pair for each ColorCombination variant
    pub fn get_pair(&self) -> Pair {
        match self {
            Callout::Success => Pair {
                fg: Colour { r: 0, g: 255, b: 0 }, // Green foreground
                bg: Colour { r: 0, g: 0, b: 0 },   // Black background
            },
            Callout::Info => Pair {
                fg: Colour {
                    r: 0,
                    g: 255,
                    b: 255,
                }, // Cyan foreground
                bg: Colour { r: 0, g: 0, b: 0 }, // Black background
            },
            Callout::Warning => Pair {
                fg: Colour {
                    r: 255,
                    g: 255,
                    b: 0,
                }, // Yellow foreground
                bg: Colour { r: 0, g: 0, b: 0 }, // Black background
            },
            Callout::Error => Pair {
                fg: Colour { r: 255, g: 0, b: 0 }, // Red foreground
                bg: Colour { r: 0, g: 0, b: 0 },   // Black background
            },
            Callout::Disabled => Pair {
                fg: Colour {
                    r: 128,
                    g: 128,
                    b: 128,
                }, // Gray foreground (disabled)
                bg: Colour { r: 0, g: 0, b: 0 }, // Black background
            },
        }
    }
}

impl From<Callout> for Style {
    /// gets you the style object directly. sets bg and fg
    fn from(val: Callout) -> Self {
        let pair = val.get_pair();
        let style: Style = pair.fg.into();
        let Colour { r, g, b } = pair.bg;
        style.bg(style::Color::Rgb(r, g, b))
    }
}
