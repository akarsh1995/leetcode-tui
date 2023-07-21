use crate::{
    app_ui::components::help_text::HelpText,
    entities::{QuestionModel, TopicTagModel},
};

#[derive(Debug, Clone)]
pub struct PopupMessage {
    pub(crate) title: String,
    pub(crate) message: String,
}

#[derive(Debug, Hash, Eq, Clone, PartialEq)]
pub enum WidgetName {
    QuestionList,
    TopicList,
}

#[derive(Debug, Clone)]
pub enum Notification {
    Questions(WidgetName, Vec<TopicTagModel>),
    Stats(WidgetName, Vec<QuestionModel>),
    Popup(WidgetName, PopupMessage),
    HelpText(WidgetName, Vec<HelpText>),
}

impl Notification {
    pub fn get_wid_name(&self) -> &WidgetName {
        match self {
            Notification::Questions(n, _) => n,
            Notification::Stats(n, _) => n,
            Notification::Popup(n, _) => n,
            Notification::HelpText(n, _) => n,
        }
    }
}

#[derive(Debug)]
pub enum WidgetVariant {
    QuestionList(QuestionListWidget),
    TopicList(TopicTagListWidget),
}

pub use crossbeam::channel::unbounded as notification_channel;

use super::{question_list::QuestionListWidget, topic_list::TopicTagListWidget};

pub type NotificationRequestSender = crossbeam::channel::Sender<Notification>;
pub type NotificationRequestReceiver = crossbeam::channel::Receiver<Notification>;

pub type NotificationRequestSendError = crossbeam::channel::SendError<Notification>;
