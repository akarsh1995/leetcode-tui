pub mod color;
pub mod style;

use crate::theme::style::Style;
use serde::{Deserialize, Serialize};

use self::color::Color;

#[derive(Serialize, Debug, Deserialize)]
pub struct Difficulty {
    pub easy: Style,
    pub medium: Style,
    pub hard: Style,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Question {
    pub normal: Difficulty,
    pub hovered: Difficulty,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Topic {
    pub normal: Style,
    pub hovered: Style,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Border {
    pub normal: Style,
    pub hovered: Style,
}

#[derive(Serialize, Debug, Deserialize)]
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

#[derive(Serialize, Debug, Deserialize)]
pub struct Theme {
    pub question: Question,
    pub topic: Topic,
    pub border: Border,
    pub defaults: Defaults,
}

impl Default for Theme {
    fn default() -> Self {
        toml::from_str(
            r#"
            [question.normal]
            easy = { fg = '#9ece6a' }
            medium = { fg = '#e0af68' }
            hard = { fg = '#f7768e' }

            [question.hovered]
            easy = { fg = '#9ece6a', bg = '#292e42', bold = true }
            medium = { fg = '#e0af68', bg = '#292e42', bold = true }
            hard = { fg = '#f7768e', bg = '#292e42', bold = true }

            [topic]
            normal = { fg = '#a9b1d6' }
            hovered = { fg = '#c0caf5', bg = '#292e42', bold = true }

            [border]
            hovered = { fg = '#7dcfff', bold = true }
            normal = { fg = '#a9b1d6' }

            [defaults]
            bg_dark = '#1f2335'
            bg = '#24283b'
            bg_highlight = '#292e42'
            terminal_black = '#414868'
            fg = '#c0caf5'
            fg_dark = '#a9b1d6'
            fg_gutter = '#3b4261'
            dark3 = '#545c7e'
            comment = '#565f89'
            dark5 = '#737aa2'
            info = '#7dcfff'
        "#,
        )
        .unwrap()
    }
}
