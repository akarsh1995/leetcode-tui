pub mod query {
    use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

    use crate::entities::question::Model as QuestionModel;
    use crate::entities::topic_tag::Model as TopicTagModel;
    use crate::entities::{prelude::*, topic_tag};

    pub async fn get_questions_by_topic(
        conn: &DatabaseConnection,
        topic_tag: &str,
    ) -> Vec<(TopicTagModel, Vec<QuestionModel>)> {
        TopicTag::find()
            .filter(topic_tag::Column::Name.contains(topic_tag))
            .find_with_related(Question)
            .all(conn)
            .await
            .unwrap()
    }

    pub async fn get_all_topics(conn: &DatabaseConnection) -> Vec<TopicTagModel> {
        TopicTag::find().all(conn).await.unwrap()
    }

    #[cfg(test)]
    pub mod tests {
        use sea_orm::Database;

        use super::*;

        #[tokio::test]
        async fn test_fetch() {
            let database_client = Database::connect("sqlite://leetcode.sqlite").await.unwrap();
            let q = get_questions_by_topic(&database_client, "array").await;
            dbg!(q);
        }
    }
}
