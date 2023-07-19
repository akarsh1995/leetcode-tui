use crate::entities::{
    question::ActiveModel as QuestionActiveModel,
    question_topic_tag::ActiveModel as QuestionTopicActiveModel,
    question_topic_tag::Model as QuestionTopicTagModel,
    topic_tag::ActiveModel as TopicTagActiveModel,
};
use crate::errors::AppResult;
use crate::{
    deserializers::question::{Question, TopicTag},
    entities::{
        prelude::QuestionTopicTag,
        prelude::TopicTag as TopicTagEntity,
        question::Entity as QuestionEntity,
        question::{self, Model as QuestionModel},
        topic_tag::{self, Model as TopicTagModel},
    },
};

use super::ModelUtils;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{prelude::*, sea_query::OnConflict};

#[async_trait]
impl ModelUtils for TopicTag {
    type ActiveModel = TopicTagActiveModel;
    type Entity = TopicTagEntity;
    type Model = TopicTagModel;

    fn on_conflict() -> OnConflict {
        OnConflict::column(topic_tag::Column::Id)
            .update_columns([topic_tag::Column::Name, topic_tag::Column::Slug])
            .to_owned()
    }
}

#[async_trait]
impl ModelUtils for Question {
    type ActiveModel = QuestionActiveModel;
    type Entity = QuestionEntity;
    type Model = QuestionModel;

    fn on_conflict() -> OnConflict {
        OnConflict::column(question::Column::FrontendQuestionId)
            .update_columns([
                question::Column::Status,
                question::Column::Title,
                question::Column::Difficulty,
                question::Column::IsFavor,
                question::Column::AcRate,
            ])
            .to_owned()
    }

    async fn post_multi_insert(db: &DatabaseConnection, objects: Vec<Self>) -> AppResult<()> {
        let mut qtags: Vec<QuestionTopicActiveModel> = vec![];

        for quest in objects {
            let qid = quest.frontend_question_id;
            if let Some(tts) = quest.topic_tags {
                for tt in &tts {
                    let tt_id = tt.id.clone();
                    qtags.push(
                        QuestionTopicTagModel {
                            question_id: qid.clone(),
                            tag_id: tt_id,
                        }
                        .into(),
                    )
                }
                TopicTag::multi_insert(db, tts).await?;
            }
        }

        let qtt_insert_result = QuestionTopicTag::insert_many(qtags).exec(db).await;

        if let Err(DbErr::RecordNotInserted) = qtt_insert_result {
            println!("Some records not inserted because they are already present.")
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::deserializers::question::ProblemSetQuestionListQuery;
    use sea_orm::Database;
    // refactor to create mock db tests
    #[tokio::test]
    async fn test() {
        let database_client = Database::connect("sqlite://leetcode.sqlite").await.unwrap();
        let json = r#"{ "data": { "problemsetQuestionList": { "total": 2777, "questions": [ { "acRate": 45.35065222510613, "difficulty": "Medium", "freqBar": null, "frontendQuestionId": "6", "isFavor": false, "paidOnly": false, "status": "ac", "title": "Zigzag Conversion", "titleSlug": "zigzag-conversion", "topicTags": [ { "name": "String", "id": "VG9waWNUYWdOb2RlOjEw", "slug": "string" } ], "hasSolution": true, "hasVideoSolution": false } ] } } }"#;
        let ppp: ProblemSetQuestionListQuery = serde_json::from_str(json).unwrap();
        let questions = ppp.get_questions();
        Question::multi_insert(&database_client, questions)
            .await
            .unwrap();
    }
}
