use crate::entities::{
    question, question_topic_tag,
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
                .do_nothing()
                .to_owned(),
        );
        d.exec(db).await.unwrap();

        let j = QuestionTopicTag::insert_many(ques.get_question_topics_relation());
        if let Err(DbErr::RecordNotInserted) = j.exec(db).await {
            println!("Some records not inserted because they are already present.")
        };
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    use super::*;
    // refactor to create mock db tests
    #[tokio::test]
    async fn test() {
        let database_client = Database::connect("sqlite://leetcode.sqlite").await.unwrap();
        get(&database_client).await;
    }
}
