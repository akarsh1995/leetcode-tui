use sea_orm::DatabaseConnection;

use crate::app_ui::channel::{Response, TaskResponse};
use crate::app_ui::widgets::notification::WidgetName;
use crate::entities::TopicTagEntity;
use crate::graphql::editor_data::Query as QuestionEditorDataQuery;
use crate::graphql::question_content::Query as QuestionGQLQuery;
use crate::graphql::GQLLeetcodeQuery;

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
