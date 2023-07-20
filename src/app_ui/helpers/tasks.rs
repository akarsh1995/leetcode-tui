use sea_orm::DatabaseConnection;

use crate::app_ui::channel::{Response, TaskResponse};
use crate::entities::TopicTagEntity;
use crate::graphql::question_content::Query as QuestionGQLQuery;
use crate::graphql::GQLLeetcodeQuery;

pub async fn get_question_details(
    sender_id: i32,
    slug: String,
    client: &reqwest::Client,
) -> TaskResponse {
    match QuestionGQLQuery::new(slug).post(client).await {
        Ok(resp) => {
            let query_response = resp;
            TaskResponse::QuestionDetail(Response {
                content: query_response.data.question,
                sender_id,
            })
        }
        Err(e) => TaskResponse::Error(Response {
            content: e.to_string(),
            sender_id,
        }),
    }
}

pub async fn get_all_questions(sender_id: i32, conn: &DatabaseConnection) -> TaskResponse {
    match TopicTagEntity::get_all_topic_questions_map(conn).await {
        Ok(map) => TaskResponse::GetAllQuestionsMap(Response {
            content: map,
            sender_id,
        }),
        Err(e) => TaskResponse::Error(Response {
            content: e.to_string(),
            sender_id,
        }),
    }
}

pub async fn get_all_topic_tags(sender_id: i32, conn: &DatabaseConnection) -> TaskResponse {
    match TopicTagEntity::get_all_topics(conn).await {
        Ok(t_tags) => TaskResponse::AllTopicTags(Response {
            content: t_tags,
            sender_id,
        }),
        Err(e) => TaskResponse::Error(Response {
            content: e.to_string(),
            sender_id,
        }),
    }
}
