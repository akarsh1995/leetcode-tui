use std::collections::HashMap;

use crate::app_ui::helpers::tasks::*;
use crate::{
    deserializers,
    entities::{QuestionModel, TopicTagModel},
};

#[derive(Debug)]
pub enum TaskRequest {
    QuestionDetail {
        slug: String,
        widget_name: WidgetName,
    },
    GetAllQuestionsMap {
        widget_name: WidgetName,
    },
    GetAllTopicTags {
        widget_name: WidgetName,
    },
}

impl TaskRequest {
    pub async fn execute(
        self,
        client: &reqwest::Client,
        conn: &DatabaseConnection,
    ) -> TaskResponse {
        match self {
            TaskRequest::QuestionDetail { slug, widget_name } => {
                get_question_details(widget_name, slug, client).await
            }
            TaskRequest::GetAllQuestionsMap { widget_name } => {
                get_all_questions(widget_name, conn).await
            }
            TaskRequest::GetAllTopicTags { widget_name } => {
                get_all_topic_tags(widget_name, conn).await
            }
        }
    }
}

#[derive(Debug)]
pub struct Response<T> {
    pub(crate) content: T,
    pub(crate) widget_name: WidgetName,
}

#[derive(Debug)]
pub enum TaskResponse {
    QuestionDetail(Response<deserializers::question_content::QuestionContent>),
    GetAllQuestionsMap(Response<HashMap<TopicTagModel, Vec<QuestionModel>>>),
    AllTopicTags(Response<Vec<TopicTagModel>>),
    Error(Response<String>),
}

impl TaskResponse {
    pub fn get_widget_name(&self) -> WidgetName {
        match self {
            TaskResponse::QuestionDetail(Response {
                widget_name,
                content: _,
            }) => widget_name,
            TaskResponse::GetAllQuestionsMap(Response {
                widget_name,
                content: _,
            }) => widget_name,
            TaskResponse::AllTopicTags(Response {
                content: _,
                widget_name,
            }) => widget_name,
            TaskResponse::Error(Response {
                widget_name,
                content: _,
            }) => widget_name,
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

use super::widgets::notification::WidgetName;

pub type RequestSendError = tokio::sync::mpsc::error::SendError<TaskRequest>;
pub type RequestRecvError = tokio::sync::mpsc::error::TryRecvError;

pub type ResponseSendError = crossbeam::channel::SendError<TaskResponse>;
pub type ResponseReceiveError = crossbeam::channel::RecvError;

// pub type
