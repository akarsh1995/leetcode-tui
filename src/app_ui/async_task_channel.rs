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
    DbUpdateQuestion(Request<QuestionModel>),
}

macro_rules! impl_task_request {
    ($(($var:ident, $f_name: ident)),*) => {
        impl TaskRequest {
            pub async fn execute(
                self,
                client: &reqwest::Client,
                conn: &DatabaseConnection,
            ) -> TaskResponse {
                match self {
                    $(
                        TaskRequest::$var(Request {
                            content,
                            widget_name,
                            request_id,
                        }) => $f_name(request_id, widget_name, content, client, conn).await,
                    )*
                }
            }
        }
    };
}

impl_task_request!(
    (QuestionDetail, get_question_details),
    (GetAllQuestionsMap, get_all_questions),
    (GetAllTopicTags, get_all_topic_tags),
    (GetQuestionEditorData, get_editor_data),
    (CodeRunRequest, run_or_submit_question),
    (DbUpdateQuestion, update_status_to_accepted)
);

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
    DbUpdateStatus(Response<()>),
    Error(Response<String>),
}

macro_rules! impl_task_response {
    ($($variant:ident),*) => {
        impl TaskResponse {
            pub fn get_widget_name(&self) -> WidgetName {
                match self {
                    $(
                        TaskResponse::$variant(Response { widget_name, .. }) => widget_name,
                    )*
                }
                .clone()
            }
        }
    };
}

impl_task_response!(
    QuestionDetail,
    GetAllQuestionsMap,
    AllTopicTags,
    QuestionEditorData,
    RunResponseData,
    DbUpdateStatus,
    Error
);

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
