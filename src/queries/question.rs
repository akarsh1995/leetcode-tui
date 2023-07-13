use crate::entities::{
    question,
    topic_tag::{self, Model as TopicTagModel},
};

use super::super::entities::prelude::*;
use crate::deserializers::question::ProblemSetQuestionListRoot as PSQ;
use sea_orm::{prelude::*, sea_query::OnConflict};

async fn get(db: &DatabaseConnection) {
    let s = r#"{"data":{"problemsetQuestionList":{"total":2777,"questions":[{"acRate":50.194408705463644,"difficulty":"Easy","freqBar":null,"frontendQuestionId":"1","isFavor":false,"paidOnly":false,"status":null,"title":"Two Sum","titleSlug":"two-sum","topicTags":[{"name":"Array","id":"VG9waWNUYWdOb2RlOjU=","slug":"array"},{"name":"Hash Table","id":"VG9waWNUYWdOb2RlOjY=","slug":"hash-table"}],"hasSolution":true,"hasVideoSolution":true}]}}}"#;
    let question_root: PSQ = serde_json::from_str(s).unwrap();
    let q = question_root.data.problemset_question_list.questions;

    for ques in q {
        let c = Question::insert(ques.get_question_active_model()).on_conflict(
            OnConflict::column(question::Column::FrontendQuestionId)
                .update_columns([
                    question::Column::Status,
                    question::Column::Title,
                    question::Column::Difficulty,
                    question::Column::IsFavor,
                    question::Column::AcRate,
                ])
                .to_owned(),
        );

        c.exec(db).await.unwrap();
        let d = TopicTag::insert_many(ques.get_topic_tags_active_model()).on_conflict(
            OnConflict::column(topic_tag::Column::Id)
                .update_columns([topic_tag::Column::Name, topic_tag::Column::Slug])
                .to_owned(),
        );
        d.exec(db).await.unwrap();

        let j = QuestionTopicTag::insert_many(ques.get_question_topics_relation());
        j.exec(db).await.unwrap();
    }
}

async fn get_all_tags(db: &DatabaseConnection) -> Vec<TopicTagModel> {
    TopicTag::find().all(db).await.unwrap()
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    use super::*;
    #[tokio::test]
    async fn test() {
        let database_client = Database::connect("sqlite://leetcode.sqlite").await.unwrap();
        get(&database_client).await;
    }
}
