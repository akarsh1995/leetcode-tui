use crate::deserializers;

pub enum Request {
    QuestionDetail { slug: String },
}

pub enum Response {
    QuestionDetail(deserializers::question_content::QuestionContent),
}

pub type ChannelRequestSender = tokio::sync::mpsc::UnboundedSender<Request>;
pub type ChannelRequestReceiver = tokio::sync::mpsc::UnboundedReceiver<Request>;

pub use tokio::sync::mpsc::unbounded_channel as request_channel;

pub type ChannelResponseSender = crossbeam::channel::Sender<Response>;
pub type ChannelResponseReceiver = crossbeam::channel::Receiver<Response>;

pub use crossbeam::channel::unbounded as response_channel;
