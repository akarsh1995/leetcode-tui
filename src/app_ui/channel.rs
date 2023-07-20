use std::collections::HashMap;

use crate::app_ui::helpers::tasks::*;
use crate::{
    deserializers,
    entities::{QuestionModel, TopicTagModel},
};

#[derive(Debug)]
pub struct Request<T> {
    pub sender_id: i32,
    pub message: T,
}

#[derive(Debug)]
pub enum TaskRequest {
    QuestionDetail { slug: String, sender_id: i32 },
    GetAllQuestionsMap { sender_id: i32 },
    GetAllTopicTags { sender_id: i32 },
}

impl TaskRequest {
    pub async fn execute(
        self,
        client: &reqwest::Client,
        conn: &DatabaseConnection,
    ) -> TaskResponse {
        match self {
            TaskRequest::QuestionDetail { slug, sender_id } => {
                get_question_details(sender_id, slug, client).await
            }
            TaskRequest::GetAllQuestionsMap { sender_id } => {
                get_all_questions(sender_id, conn).await
            }
            TaskRequest::GetAllTopicTags { sender_id } => get_all_topic_tags(sender_id, conn).await,
        }
    }
}

#[derive(Debug)]
pub struct Response<T> {
    pub(crate) content: T,
    pub(crate) sender_id: i32,
}

#[derive(Debug)]
pub enum TaskResponse {
    QuestionDetail(Response<deserializers::question_content::QuestionContent>),
    GetAllQuestionsMap(Response<HashMap<TopicTagModel, Vec<QuestionModel>>>),
    AllTopicTags(Response<Vec<TopicTagModel>>),
    Error(Response<String>),
}

impl TaskResponse {
    pub fn get_sender_id(&self) -> i32 {
        match self {
            TaskResponse::QuestionDetail(Response {
                sender_id,
                content: _,
            }) => sender_id,
            TaskResponse::GetAllQuestionsMap(Response {
                sender_id,
                content: _,
            }) => sender_id,
            TaskResponse::AllTopicTags(Response {
                sender_id,
                content: _,
            }) => sender_id,
            TaskResponse::Error(Response {
                sender_id,
                content: _,
            }) => sender_id,
        }
        .clone()
    }
}

pub type ChannelRequestSender = tokio::sync::mpsc::UnboundedSender<TaskRequest>;
pub type ChannelRequestReceiver = tokio::sync::mpsc::UnboundedReceiver<TaskRequest>;

use sea_orm::DatabaseConnection;
pub use tokio::sync::mpsc::unbounded_channel as request_channel;

pub type ChannelResponseSender = crossbeam::channel::Sender<TaskResponse>;
pub type ChannelResponseReceiver = crossbeam::channel::Receiver<TaskResponse>;

pub use crossbeam::channel::unbounded as response_channel;

pub type RequestSendError = tokio::sync::mpsc::error::SendError<TaskRequest>;
pub type RequestRecvError = tokio::sync::mpsc::error::TryRecvError;

pub type ResponseSendError = crossbeam::channel::SendError<TaskResponse>;
pub type ResponseReceiveError = crossbeam::channel::RecvError;

// pub type
