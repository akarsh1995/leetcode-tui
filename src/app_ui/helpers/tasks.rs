use sea_orm::DatabaseConnection;

use crate::app_ui::channel::{Response, TaskResponse};
use crate::app_ui::widgets::notification::WidgetName;
use crate::entities::TopicTagEntity;
use crate::graphql::question_content::Query as QuestionGQLQuery;
use crate::graphql::GQLLeetcodeQuery;

pub async fn get_question_details(
    widget_name: WidgetName,
    slug: String,
    client: &reqwest::Client,
) -> TaskResponse {
    match QuestionGQLQuery::new(slug).post(client).await {
        Ok(resp) => {
            let query_response = resp;
            TaskResponse::QuestionDetail(Response {
                content: query_response.data.question,
                widget_name,
            })
        }
        Err(e) => TaskResponse::Error(Response {
            content: e.to_string(),
            widget_name,
        }),
    }
}

pub async fn get_all_questions(widget_name: WidgetName, conn: &DatabaseConnection) -> TaskResponse {
    match TopicTagEntity::get_all_topic_questions_map(conn).await {
        Ok(map) => TaskResponse::GetAllQuestionsMap(Response {
            content: map,
            widget_name,
        }),
        Err(e) => TaskResponse::Error(Response {
            content: e.to_string(),
            widget_name,
        }),
    }
}

pub async fn get_all_topic_tags(
    widget_name: WidgetName,
    conn: &DatabaseConnection,
) -> TaskResponse {
    match TopicTagEntity::get_all_topics(conn).await {
        Ok(t_tags) => TaskResponse::AllTopicTags(Response {
            content: t_tags,
            widget_name,
        }),
        Err(e) => TaskResponse::Error(Response {
            content: e.to_string(),
            widget_name,
        }),
    }
}
