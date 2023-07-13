use super::super::entities::prelude::*;
use crate::deserializers::topic_tag::ProblemSetQuestionListRoot as PSQ;
use crate::entities::topic_tag::ActiveModel as AV;
use crate::{
    deserializers::question::ProblemSetQuestionListRoot, entities::question::ActiveModel as AD,
};
use sea_orm::prelude::*;

async fn get(db: &DatabaseConnection) {
    let s = r#"{"data":{"problemsetQuestionList":{"total":2777,"questions":[{"acRate":50.194408705463644,"difficulty":"Easy","freqBar":null,"frontendQuestionId":"1","isFavor":false,"paidOnly":false,"status":null,"title":"Two Sum","titleSlug":"two-sum","topicTags":[{"name":"Array","id":"VG9waWNUYWdOb2RlOjU=","slug":"array"},{"name":"Hash Table","id":"VG9waWNUYWdOb2RlOjY=","slug":"hash-table"}],"hasSolution":true,"hasVideoSolution":true}]}}}"#;
    let mut topics_root: PSQ = serde_json::from_str(s).unwrap();
    let questions_topics = topics_root.get_questions_with_topics();
    let mut question_root: ProblemSetQuestionListRoot = serde_json::from_str(s).unwrap();
    let q = question_root.get_questions();
    let mut v = vec![];
    while !q.is_empty() {
        let active_question: AD = q.pop().unwrap().into();
        v.push(active_question)
    }

    let mut v2 = vec![];
    while !questions_topics.is_empty() {
        let active_question_topics: AV = questions_topics
            .pop()
            .unwrap()
            .topic_tags
            .pop()
            .unwrap()
            .into();
        v2.push(active_question_topics)
    }

    let c = Question::insert_many(v);
    c.exec(db).await.unwrap();
    let d = TopicTag::insert_many(v2);
    d.exec(db).await.unwrap();
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
