use std::collections::HashMap;

use crate::app_ui::helpers::tasks::*;
use crate::{
    deserializers,
    entities::{QuestionModel, TopicTagModel},
};

#[derive(Debug)]
pub enum TaskRequest {
    QuestionDetail { slug: String },
    GetAllQuestionsMap,
    GetAllTopicTags,
}

impl TaskRequest {
    pub async fn execute(
        self,
        client: &reqwest::Client,
        conn: &DatabaseConnection,
    ) -> TaskResponse {
        match self {
            TaskRequest::QuestionDetail { slug } => get_question_details(slug, client).await,
            TaskRequest::GetAllQuestionsMap => get_all_questions(conn).await,
            TaskRequest::GetAllTopicTags => get_all_topic_tags(conn).await,
        }
    }
}

#[derive(Debug)]
pub enum TaskResponse {
    QuestionDetail(deserializers::question_content::QuestionContent),
    GetAllQuestionsMap(HashMap<TopicTagModel, Vec<QuestionModel>>),
    AllTopicTags(Vec<TopicTagModel>),
    Error(String),
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
