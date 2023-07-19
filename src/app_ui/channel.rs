use crate::deserializers;
use crate::errors::AppResult;

#[derive(Debug)]
pub enum TaskRequest {
    QuestionDetail { slug: String },
}

#[derive(Debug)]
pub enum TaskResponse {
    QuestionDetail(deserializers::question_content::QuestionContent),
    Error(String),
}

pub type ChannelRequestSender = tokio::sync::mpsc::UnboundedSender<TaskRequest>;
pub type ChannelRequestReceiver = tokio::sync::mpsc::UnboundedReceiver<TaskRequest>;

pub use tokio::sync::mpsc::unbounded_channel as request_channel;

pub type ChannelResponseSender = crossbeam::channel::Sender<TaskResponse>;
pub type ChannelResponseReceiver = crossbeam::channel::Receiver<TaskResponse>;

pub use crossbeam::channel::unbounded as response_channel;

pub type RequestSendError = tokio::sync::mpsc::error::SendError<TaskRequest>;
pub type RequestRecvError = tokio::sync::mpsc::error::TryRecvError;

pub type ResponseSendError = crossbeam::channel::SendError<TaskResponse>;
pub type ResponseReceiveError = crossbeam::channel::RecvError;

// pub type
