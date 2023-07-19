use std::collections::HashMap;

use crate::entities::question::Model as QuestionModel;
use crate::entities::topic_tag::Model as TopicTagModel;
use crate::entities::{prelude::*, topic_tag};
use crate::errors::AppResult;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

impl TopicTag {
    pub async fn get_questions_by_topic(
        conn: &DatabaseConnection,
        topic_tag: &str,
    ) -> AppResult<Vec<(TopicTagModel, Vec<QuestionModel>)>> {
        Ok(TopicTag::find()
            .filter(topic_tag::Column::Name.contains(topic_tag))
            .find_with_related(Question)
            .all(conn)
            .await?)
    }

    pub async fn get_all_topics(conn: &DatabaseConnection) -> AppResult<Vec<TopicTagModel>> {
        Ok(TopicTag::find().all(conn).await?)
    }

    pub async fn get_all_topic_questions_map(
        conn: &DatabaseConnection,
    ) -> AppResult<HashMap<TopicTagModel, Vec<QuestionModel>>> {
        let topic_tag_quests = Self::get_questions_by_topic(conn, "").await?;
        Ok(topic_tag_quests
            .into_iter()
            .map(|(tt, qs)| (tt, qs))
            .collect::<HashMap<TopicTagModel, Vec<QuestionModel>>>())
    }
}
