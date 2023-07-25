use crate::{
    app_ui::components::help_text::HelpText,
    entities::{QuestionModel, TopicTagModel},
};

#[derive(Debug, Clone)]
pub struct PopupMessage {
    pub(crate) title: String,
    pub(crate) message: String,
    pub(crate) help_texts: IndexSet<HelpText>,
}

#[derive(Debug, Hash, Eq, Clone, PartialEq)]
pub enum WidgetName {
    QuestionList,
    TopicList,
    Stats,
    Popup,
    HelpLine,
}

#[derive(Debug, Clone)]
pub struct NotifContent<T> {
    pub src_wid: WidgetName,
    pub dest_wid: WidgetName,
    pub content: T,
}

impl<T> NotifContent<T> {
    pub fn new(src_wid: WidgetName, dest_wid: WidgetName, content: T) -> Self {
        Self {
            src_wid,
            dest_wid,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Notification {
    Questions(NotifContent<Vec<TopicTagModel>>),
    Stats(NotifContent<Vec<QuestionModel>>),
    Popup(NotifContent<PopupMessage>),
    HelpText(NotifContent<IndexSet<HelpText>>),
    Event(NotifContent<KeyEvent>),
}

macro_rules! dest_widname {
    ($($variant:ident),*) => {
        pub fn get_wid_name(&self) -> &WidgetName {
            match self {
                $(
                    Notification::$variant(NotifContent {
                        src_wid: _,
                        dest_wid,
                        content: _,
                    }) => dest_wid,
                )*
            }
        }
    };
}

impl Notification {
    dest_widname!(Questions, Stats, Popup, HelpText, Event);
}

#[derive(Debug)]
pub enum WidgetVariant {
    QuestionList(QuestionListWidget),
    TopicList(TopicTagListWidget),
    Stats(Stats),
    Popup(Popup),
    HelpLine(Footer),
}

pub use crossbeam::channel::unbounded as notification_channel;
use crossterm::event::KeyEvent;
use indexmap::IndexSet;

use super::{
    footer::Footer, popup::Popup, question_list::QuestionListWidget, stats::Stats,
    topic_list::TopicTagListWidget,
};
