// ┌─────────────────────────────────────────────────────────────────────────┐
// │                                  Leetui                                 │
// │  ┌─Topics─────┐ ┌─Questions──────────────────────────────────────────┐  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │          ┌─┘Popup────────────────────────────────────┐            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          │                                           │            │  │
// │  │          └─┬─┬───────────────────────────────────────┘            │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  └────────────┘ └────────────────────────────────────────────────────┘  │
// │  /search                                                      [?] Help  │
// └─────────────────────────────────────────────────────────────────────────┘

// ┌─────────────────────────────────────────────────────────────────────────┐
// │                                  Leetui                                 │
// │  ┌─Topics─────┐ ┌─Stats──────────────────────────────────────────────┐  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  │            │ │                                                    │  │
// │  └────────────┘ └────────────────────────────────────────────────────┘  │
// │  /search                                                      [?] Help  │
// └─────────────────────────────────────────────────────────────────────────┘

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Widget},
};

#[derive(Debug, Clone)]
pub struct BlockAreas {
    pub inner: Rect,
    pub outer: Rect,
}

impl From<Rect> for BlockAreas {
    fn from(value: Rect) -> Self {
        Self {
            outer: value,
            inner: value.blockify(),
        }
    }
}

#[derive(Debug)]
pub struct Window {
    pub root: Root,
}

#[derive(Debug)]
pub struct StatusBar {
    pub search_area: Rect,
    pub message_area: Rect,
}

impl From<Rect> for StatusBar {
    fn from(value: Rect) -> Self {
        let left_and_right_div = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(value);
        Self {
            search_area: left_and_right_div[0],
            message_area: left_and_right_div[1],
        }
    }
}

#[derive(Debug)]
pub struct Root {
    pub top_bar: Rect,
    pub center_layout: CenterLayout,
    pub status_bar: StatusBar,
    pub popup: BlockAreas,
    pub q_stats: BlockAreas,
}

#[derive(Debug)]
pub struct CenterLayout {
    pub question: BlockAreas,
    pub topic: BlockAreas,
}

impl CenterLayout {
    fn new(chunks: Rect) -> Self {
        let center_chunks = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(20), Constraint::Percentage(80)],
        )
        .split(chunks);

        let topic_area = center_chunks[0];
        let question_area = center_chunks[1];
        Self {
            question: question_area.into(),
            topic: topic_area.into(),
        }
    }
}

impl Root {
    fn new(ar: Rect) -> Self {
        let chunks = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ],
        )
        .split(ar);

        let mut r = Self {
            top_bar: chunks[0],
            center_layout: CenterLayout::new(chunks[1]),
            status_bar: chunks[2].into(),
            popup: centered_rect(60, 60, ar).into(),
            q_stats: centered_rect(60, 60, ar).into(),
        };
        r.q_stats = r.center_layout.question.clone();
        r
    }
}

impl Default for Window {
    fn default() -> Self {
        let term_size = super::tui::Term::size();
        let window_rect = Rect::new(0, 0, term_size.columns, term_size.rows);
        Self {
            root: Root::new(window_rect),
        }
    }
}

pub trait GetWindowStats {
    fn get_window(&self) -> Window {
        Window::default()
    }
}

impl<T> GetWindowStats for T where T: Widget {}

trait Blockify {
    fn blockify(self) -> Rect;
}

impl Blockify for Rect {
    fn blockify(self) -> Rect {
        Block::default().borders(Borders::ALL).inner(self)
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
