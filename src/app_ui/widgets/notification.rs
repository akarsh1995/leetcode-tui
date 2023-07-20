use crate::entities::{QuestionModel, TopicTagModel};

#[derive(Debug, Clone)]
pub enum Notification {
    UpdateQuestions(Vec<TopicTagModel>),
    UpdateStats(Vec<QuestionModel>),
}

pub use crossbeam::channel::unbounded as notification_channel;

pub type NotificationRequestSender = crossbeam::channel::Sender<Notification>;
pub type NotificationRequestReceiver = crossbeam::channel::Receiver<Notification>;

pub type NotificationRequestSendError = crossbeam::channel::SendError<Notification>;
