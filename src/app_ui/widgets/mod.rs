// pub mod footer;
pub(crate) mod notification;
pub(crate) mod popup;
pub mod question_list;
pub mod stats;
pub mod topic_list;

use std::{collections::HashMap, fmt::Debug, io::Stderr};

use crossterm::event::KeyEvent;
use ratatui::{prelude::Rect, prelude::*, Frame};

use crate::errors::AppResult;

use self::notification::{Notification, WidgetName, WidgetVariant};

use super::channel::{ChannelRequestSender, TaskResponse};

#[derive(Debug)]
pub struct CommonState {
    pub widget_name: WidgetName,
    active: bool,
    pub task_sender: ChannelRequestSender,
    pub is_navigable: bool,
}

impl CommonState {
    pub(crate) fn new(id: WidgetName, task_sender: ChannelRequestSender) -> Self {
        Self {
            widget_name: id,
            active: false,
            task_sender,
            is_navigable: true,
        }
    }
}

pub trait Widget: Debug {
    fn set_active(&mut self) {
        self.get_common_state_mut().active = true;
    }
    fn is_active(&self) -> bool {
        self.get_common_state().active
    }

    fn is_navigable(&self) -> bool {
        self.get_common_state().is_navigable
    }

    fn set_inactive(&mut self) {
        self.get_common_state_mut().active = false;
    }

    fn get_widget_name(&self) -> WidgetName {
        self.get_common_state().widget_name.clone()
    }
    fn get_task_sender(&self) -> &ChannelRequestSender {
        &self.get_common_state().task_sender
    }

    fn get_common_state_mut(&mut self) -> &mut CommonState;

    fn get_common_state(&self) -> &CommonState;

    fn render(&mut self, rect: Rect, frame: &mut Frame<CrosstermBackend<Stderr>>);

    fn handler(&mut self, event: KeyEvent) -> AppResult<Option<Notification>>;

    fn process_task_response(&mut self, response: TaskResponse) -> AppResult<Option<Notification>>;

    fn setup(&mut self) -> AppResult<Option<Notification>> {
        Ok(None)
    }

    fn set_response(&mut self);

    fn process_notification(
        &mut self,
        notification: &Notification,
    ) -> AppResult<Option<Notification>>;
}

impl WidgetVariant {
    pub fn set_active(&mut self) {
        match self {
            WidgetVariant::QuestionList(v) => v.set_active(),
            WidgetVariant::TopicList(v) => v.set_active(),
            WidgetVariant::Stats(v) => v.set_active(),
            WidgetVariant::Popup(v) => v.set_active(),
        }
    }

    pub fn set_inactive(&mut self) {
        match self {
            WidgetVariant::QuestionList(v) => v.set_inactive(),
            WidgetVariant::TopicList(v) => v.set_inactive(),
            WidgetVariant::Stats(v) => v.set_inactive(),
            WidgetVariant::Popup(v) => v.set_inactive(),
        }
    }

    pub fn is_navigable(&self) -> bool {
        match self {
            WidgetVariant::QuestionList(v) => v.is_navigable(),
            WidgetVariant::TopicList(v) => v.is_navigable(),
            WidgetVariant::Stats(v) => v.is_navigable(),
            WidgetVariant::Popup(v) => v.is_navigable(),
        }
    }

    pub fn setup(&mut self) -> AppResult<Option<Notification>> {
        match self {
            WidgetVariant::QuestionList(v) => v.setup(),
            WidgetVariant::TopicList(v) => v.setup(),
            WidgetVariant::Stats(v) => v.setup(),
            WidgetVariant::Popup(v) => v.setup(),
        }
    }

    pub fn process_task_response(
        &mut self,
        response: TaskResponse,
    ) -> AppResult<Option<Notification>> {
        match self {
            WidgetVariant::QuestionList(v) => v.process_task_response(response),
            WidgetVariant::TopicList(v) => v.process_task_response(response),
            WidgetVariant::Stats(v) => v.process_task_response(response),
            WidgetVariant::Popup(v) => v.process_task_response(response),
        }
    }

    pub fn handler(&mut self, event: KeyEvent) -> AppResult<Option<Notification>> {
        match self {
            WidgetVariant::QuestionList(v) => v.handler(event),
            WidgetVariant::TopicList(v) => v.handler(event),
            WidgetVariant::Stats(v) => v.handler(event),
            WidgetVariant::Popup(v) => v.handler(event),
        }
    }

    pub fn process_notification(
        &mut self,
        notification: &Notification,
    ) -> AppResult<Option<Notification>> {
        match self {
            WidgetVariant::QuestionList(v) => v.process_notification(notification),
            WidgetVariant::TopicList(v) => v.process_notification(notification),
            WidgetVariant::Stats(v) => v.process_notification(notification),
            WidgetVariant::Popup(v) => v.process_notification(notification),
        }
    }

    pub fn render(&mut self, rect: Rect, frame: &mut Frame<CrosstermBackend<Stderr>>) {
        match self {
            WidgetVariant::QuestionList(v) => v.render(rect, frame),
            WidgetVariant::TopicList(v) => v.render(rect, frame),
            WidgetVariant::Stats(v) => v.render(rect, frame),
            WidgetVariant::Popup(v) => v.render(rect, frame),
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            WidgetVariant::QuestionList(v) => v.is_active(),
            WidgetVariant::TopicList(v) => v.is_active(),
            WidgetVariant::Stats(v) => v.is_active(),
            WidgetVariant::Popup(v) => v.is_active(),
        }
    }
}

pub type WidgetList = Vec<Box<dyn Widget>>;
pub type NameWidgetMap = HashMap<WidgetName, Box<dyn Widget>>;
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
