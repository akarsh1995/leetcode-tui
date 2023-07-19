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

pub mod query {
    use super::*;

    impl QuestionEntity {
        pub async fn get_question_count(db: &DatabaseConnection) -> AppResult<u64> {
            Ok(Self::find().count(db).await?)
        }
    }
}
