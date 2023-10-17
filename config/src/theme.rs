pub mod color;
pub mod style;

use crate::theme::style::Style;
use serde::Deserialize;

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
pub struct Theme {
    pub question: Question,
    pub topic: Topic,
    pub border: Border,
}
