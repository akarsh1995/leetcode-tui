use sea_orm::{prelude::*, DatabaseConnection, IntoActiveModel, Set};

use crate::app_ui::async_task_channel::{Response, TaskResponse};
use crate::app_ui::widgets::notification::WidgetName;
use crate::entities::{QuestionModel, TopicTagEntity};
use crate::graphql::editor_data::Query as QuestionEditorDataQuery;
use crate::graphql::question_content::Query as QuestionGQLQuery;
use crate::graphql::run_code::RunCode;
use crate::graphql::{self, GQLLeetcodeQuery, RunOrSubmitCode};

pub async fn get_question_details(
    request_id: String,
    widget_name: WidgetName,
    slug: String,
    client: &reqwest::Client,
) -> TaskResponse {
    match QuestionGQLQuery::new(slug).post(client).await {
        Ok(resp) => {
            let query_response = resp;
            TaskResponse::QuestionDetail(Response {
                request_id,
                content: query_response.data.question,
                widget_name,
            })
        }
        Err(e) => TaskResponse::Error(Response {
            request_id,
            content: e.to_string(),
            widget_name,
        }),
    }
}

pub async fn get_editor_data(
    request_id: String,
    widget_name: WidgetName,
    slug: String,
    client: &reqwest::Client,
) -> TaskResponse {
    match QuestionEditorDataQuery::new(slug).post(client).await {
        Ok(data) => TaskResponse::QuestionEditorData(Response {
            request_id,
            content: data.data.question,
            widget_name,
        }),
        Err(e) => TaskResponse::Error(Response {
            request_id,
            content: e.to_string(),
            widget_name,
        }),
    }
}

pub async fn get_all_questions(
    request_id: String,
    widget_name: WidgetName,
    conn: &DatabaseConnection,
) -> TaskResponse {
    match TopicTagEntity::get_all_topic_questions_map(conn).await {
        Ok(map) => TaskResponse::GetAllQuestionsMap(Response {
            request_id,
            content: map,
            widget_name,
        }),
        Err(e) => TaskResponse::Error(Response {
            request_id,
            content: e.to_string(),
            widget_name,
        }),
    }
}

pub async fn get_all_topic_tags(
    request_id: String,
    widget_name: WidgetName,
    conn: &DatabaseConnection,
) -> TaskResponse {
    match TopicTagEntity::get_all_topics(conn).await {
        Ok(t_tags) => TaskResponse::AllTopicTags(Response {
            request_id,
            content: t_tags,
            widget_name,
        }),
        Err(e) => TaskResponse::Error(Response {
            request_id,
            content: e.to_string(),
            widget_name,
        }),
    }
}

pub async fn run_or_submit_question(
    request_id: String,
    widget_name: WidgetName,
    mut run_or_submit_code: graphql::RunOrSubmitCode,
    client: &reqwest::Client,
) -> TaskResponse {
    if let RunOrSubmitCode::Run(RunCode {
        test_cases_stdin,
        slug,
        ..
    }) = &mut run_or_submit_code
    {
        match graphql::console_panel_config::Query::new(slug.clone())
            .post(client)
            .await
        {
            Ok(resp) => {
                *test_cases_stdin =
                    Some(resp.data.question.example_testcase_list.clone().join("\n"));
            }
            Err(e) => {
                return TaskResponse::Error(Response {
                    request_id,
                    content: e.to_string(),
                    widget_name,
                })
            }
        }
    }

    match run_or_submit_code.post(client).await {
        Ok(run_response_body) => TaskResponse::RunResponseData(Response {
            request_id,
            content: run_response_body,
            widget_name,
        }),
        Err(e) => TaskResponse::Error(Response {
            request_id,
            content: e.to_string(),
            widget_name,
        }),
    }
}

pub async fn update_status_to_accepted(
    request_id: String,
    widget_name: WidgetName,
    question: QuestionModel,
    db: &DatabaseConnection,
) -> TaskResponse {
    let mut am = question.into_active_model();
    am.status = Set(Some("ac".to_string()));
    match am.update(db).await {
        Ok(_) => TaskResponse::DbUpdateStatus(Response {
            request_id,
            content: (),
            widget_name,
        }),
        Err(e) => TaskResponse::Error(Response {
            request_id,
            content: e.to_string(),
            widget_name,
        }),
    }
}
