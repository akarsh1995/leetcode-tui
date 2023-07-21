use crate::entities::{QuestionModel, TopicTagModel};

#[derive(Debug, Clone)]
pub struct PopupMessage {
    pub(crate) title: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone)]
pub enum Notification {
    Questions(Vec<TopicTagModel>),
    Stats(Vec<QuestionModel>),
    Popup(PopupMessage),
}

pub use crossbeam::channel::unbounded as notification_channel;

pub type NotificationRequestSender = crossbeam::channel::Sender<Notification>;
pub type NotificationRequestReceiver = crossbeam::channel::Receiver<Notification>;

pub type NotificationRequestSendError = crossbeam::channel::SendError<Notification>;