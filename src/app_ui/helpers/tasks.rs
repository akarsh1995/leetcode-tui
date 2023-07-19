use sea_orm::DatabaseConnection;

use crate::app_ui::channel::TaskResponse;
use crate::entities::TopicTagEntity;
use crate::graphql::question_content::Query as QuestionGQLQuery;
use crate::graphql::GQLLeetcodeQuery;

pub async fn get_question_details(slug: String, client: &reqwest::Client) -> TaskResponse {
    match QuestionGQLQuery::new(slug).post(client).await {
        Ok(resp) => {
            let query_response = resp;
            TaskResponse::QuestionDetail(query_response.data.question)
        }
        Err(e) => TaskResponse::Error(e.to_string()),
    }
}

pub async fn get_all_questions(conn: &DatabaseConnection) -> TaskResponse {
    match TopicTagEntity::get_all_topic_questions_map(conn).await {
        Ok(map) => TaskResponse::GetAllQuestionsMap(map),
        Err(e) => TaskResponse::Error(e.to_string()),
    }
}

pub async fn get_all_topic_tags(conn: &DatabaseConnection) -> TaskResponse {
    match TopicTagEntity::get_all_topics(conn).await {
        Ok(t_tags) => TaskResponse::AllTopicTags(t_tags),
        Err(e) => TaskResponse::Error(e.to_string()),
    }
}
