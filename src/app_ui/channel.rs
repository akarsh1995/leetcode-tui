use crate::deserializers;
use crate::errors::AppResult;

#[derive(Debug)]
pub enum Request {
    QuestionDetail { slug: String },
}

#[derive(Debug)]
pub enum Response {
    QuestionDetail(deserializers::question_content::QuestionContent),
    Error(String),
}

pub type ChannelRequestSender = tokio::sync::mpsc::UnboundedSender<Request>;
pub type ChannelRequestReceiver = tokio::sync::mpsc::UnboundedReceiver<Request>;

pub use tokio::sync::mpsc::unbounded_channel as request_channel;

pub type ChannelResponseSender = crossbeam::channel::Sender<AppResult<Response>>;
pub type ChannelResponseReceiver = crossbeam::channel::Receiver<Response>;

pub use crossbeam::channel::unbounded as response_channel;

pub type RequestSendError = tokio::sync::mpsc::error::SendError<Request>;
pub type RequestRecvError = tokio::sync::mpsc::error::TryRecvError;

pub type ResponseSendError = crossbeam::channel::SendError<Response>;
pub type ResponseReceiveError = crossbeam::channel::RecvError;

// pub type
