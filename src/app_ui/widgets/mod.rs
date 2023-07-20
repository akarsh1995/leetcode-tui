use std::borrow::Borrow;

use ratatui::{
    prelude::*,
    prelude::{Backend, Rect},
    Frame,
};

pub mod question_list;

struct State {
    active: bool,
}

pub trait Widget {
    fn render<'a, B: Backend>(&mut self, rect: Rect, frame: &mut Frame<B>) {}
}

pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Into<Style> for Colour {
    /// sets fg color and returns the style
    fn into(self) -> Style {
        let pair = self;
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
                fg: Colour {
                    r: 255,
                    g: 255,
                    b: 255,
                }, // White foreground
                bg: Colour { r: 255, g: 0, b: 0 }, // Red background
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

impl Into<Style> for Callout {
    /// gets you the style object directly. sets bg and fg
    fn into(self) -> Style {
        let pair = self.get_pair();
        let style: Style = pair.fg.into();
        let Colour { r, g, b } = pair.bg;
        style.bg(style::Color::Rgb(r, g, b))
    }
}
