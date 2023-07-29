use std::collections::HashMap;

use crate::app_ui::helpers::tasks::*;
use crate::graphql::RunOrSubmitCode;
use crate::{
    deserializers,
    entities::{QuestionModel, TopicTagModel},
};

#[derive(Debug)]
pub struct Request<T> {
    pub(crate) request_id: String,
    pub(crate) content: T,
    pub(crate) widget_name: WidgetName,
}
pub enum TaskRequest {
    QuestionDetail(Request<String>),
    GetAllQuestionsMap(Request<()>),
    GetAllTopicTags(Request<()>),
    GetQuestionEditorData(Request<String>),
    CodeRunRequest(Request<RunOrSubmitCode>),
}

impl TaskRequest {
    pub async fn execute(
        self,
        client: &reqwest::Client,
        conn: &DatabaseConnection,
    ) -> TaskResponse {
        match self {
            TaskRequest::QuestionDetail(Request {
                content: slug,
                widget_name,
                request_id,
            }) => get_question_details(request_id, widget_name, slug, client).await,
            TaskRequest::GetAllQuestionsMap(Request {
                widget_name,
                request_id,
                ..
            }) => get_all_questions(request_id, widget_name, conn).await,
            TaskRequest::GetAllTopicTags(Request {
                widget_name,
                request_id,
                ..
            }) => get_all_topic_tags(request_id, widget_name, conn).await,
            TaskRequest::GetQuestionEditorData(Request {
                request_id,
                content,
                widget_name,
            }) => get_editor_data(request_id, widget_name, content, client).await,
            TaskRequest::CodeRunRequest(Request {
                request_id,
                content,
                widget_name,
            }) => run_or_submit_question(request_id, widget_name, content, client).await,
        }
    }
}

#[derive(Debug)]
pub struct Response<T> {
    pub(crate) request_id: String,
    pub(crate) content: T,
    pub(crate) widget_name: WidgetName,
}

#[derive(Debug)]
pub enum TaskResponse {
    QuestionDetail(Response<deserializers::question_content::QuestionContent>),
    GetAllQuestionsMap(Response<HashMap<TopicTagModel, Vec<QuestionModel>>>),
    AllTopicTags(Response<Vec<TopicTagModel>>),
    QuestionEditorData(Response<deserializers::editor_data::Question>),
    RunResponseData(Response<deserializers::run_submit::ParsedResponse>),
    Error(Response<String>),
}

impl TaskResponse {
    pub fn get_widget_name(&self) -> WidgetName {
        match self {
            TaskResponse::QuestionDetail(Response { widget_name, .. }) => widget_name,
            TaskResponse::GetAllQuestionsMap(Response { widget_name, .. }) => widget_name,
            TaskResponse::AllTopicTags(Response { widget_name, .. }) => widget_name,
            TaskResponse::Error(Response { widget_name, .. }) => widget_name,
            TaskResponse::QuestionEditorData(Response { widget_name, .. }) => widget_name,
            TaskResponse::RunResponseData(Response { widget_name, .. }) => widget_name,
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
