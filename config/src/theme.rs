pub mod color;
pub mod style;

use crate::theme::style::Style;
use serde::Deserialize;

use self::color::Color;

#[derive(Debug, Deserialize)]
pub struct Difficulty {
    pub easy: Style,
    pub medium: Style,
    pub hard: Style,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    pub normal: Difficulty,
    pub hovered: Difficulty,
}

#[derive(Debug, Deserialize)]
pub struct Topic {
    pub normal: Style,
    pub hovered: Style,
}

#[derive(Debug, Deserialize)]
pub struct Border {
    pub normal: Style,
    pub hovered: Style,
}

#[derive(Debug, Deserialize)]
pub struct Defaults {
    pub bg_dark: Color,
    pub bg: Color,
    pub bg_highlight: Color,
    pub terminal_black: Color,
    pub fg: Color,
    pub fg_dark: Color,
    pub fg_gutter: Color,
    pub dark3: Color,
    pub comment: Color,
    pub dark5: Color,
    pub info: Color,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub question: Question,
    pub topic: Topic,
    pub border: Border,
    pub defaults: Defaults,
}
