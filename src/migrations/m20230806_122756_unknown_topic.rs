use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let stmt_for_unknown_questions = Statement::from_sql_and_values(
            manager.get_database_backend(),
            r#"
                INSERT INTO TopicTag (`id`, `name`, `slug`) VALUES (?, ?, ?);

                INSERT INTO QuestionTopicTag
                SELECT DISTINCT frontend_question_id, "unknown"
                FROM Question
                LEFT JOIN QuestionTopicTag
                ON QuestionTopicTag.question_id = Question.frontend_question_id
                WHERE QuestionTopicTag.tag_id is NULL;
            "#,
            ["unknown".into(), "Unknown".into(), "unknown".into()],
        );
        db.execute(stmt_for_unknown_questions).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let stmt = Statement::from_sql_and_values(
            manager.get_database_backend(),
            r#"DELETE FROM `TopicTag` WHERE id = $1;"#,
            ["unknown".into()],
        );
        let db = manager.get_connection();
        db.execute(stmt).await?;
        Ok(())
    }
}
