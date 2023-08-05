use ratatui::prelude::*;
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from(val: (u8, u8, u8)) -> Self {
        Rgb {
            r: val.0,
            g: val.1,
            b: val.2,
        }
    }
}

impl From<Rgb> for style::Color {
    fn from(val: Rgb) -> Self {
        style::Color::Rgb(val.r, val.g, val.b)
    }
}

impl From<Rgb> for Style {
    /// sets fg color and returns the style
    fn from(val: Rgb) -> Self {
        Style::default().fg(val.into())
    }
}

pub struct Pair {
    pub fg: Rgb,
    pub bg: Rgb,
}

pub const CHECK_MARK: &str = "✔️";

pub enum TokyoNightColors {
    Foreground,
    Selection,
    Comment,
    Red,
    Orange,
    Yellow,
    Green,
    Purple,
    Cyan,
    Pink,
}

impl From<TokyoNightColors> for Rgb {
    fn from(val: TokyoNightColors) -> Self {
        match val {
            TokyoNightColors::Foreground => (192, 202, 245),
            TokyoNightColors::Selection => (40, 52, 87),
            TokyoNightColors::Comment => (86, 95, 137),
            TokyoNightColors::Red => (247, 118, 142),
            TokyoNightColors::Orange => (255, 158, 100),
            TokyoNightColors::Yellow => (224, 175, 104),
            TokyoNightColors::Green => (158, 206, 106),
            TokyoNightColors::Purple => (157, 124, 216),
            TokyoNightColors::Cyan => (125, 207, 255),
            TokyoNightColors::Pink => (187, 154, 247),
        }
        .into()
    }
}

impl From<TokyoNightColors> for Style {
    fn from(val: TokyoNightColors) -> Self {
        let color: Rgb = val.into();
        color.into()
    }
}

impl From<TokyoNightColors> for style::Color {
    fn from(val: TokyoNightColors) -> Self {
        let color: Rgb = val.into();
        color.into()
    }
}

pub enum Callout {
    Success,
    Info,
    Warning,
    Error,
    Disabled,
}

impl From<Callout> for Rgb {
    fn from(val: Callout) -> Self {
        match val {
            Callout::Success => TokyoNightColors::Green,
            Callout::Info => TokyoNightColors::Foreground,
            Callout::Warning => TokyoNightColors::Yellow,
            Callout::Error => TokyoNightColors::Red,
            Callout::Disabled => TokyoNightColors::Comment,
        }
        .into()
    }
}

impl From<Callout> for Style {
    fn from(val: Callout) -> Self {
        let color: Rgb = val.into();
        color.into()
    }
}
