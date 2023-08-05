use super::m20230718_141525_create_tables::{Question, QuestionTopicTag, TopicTag};
use sea_orm::EnumIter;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden, EnumIter)]
pub enum QuestionTemp {
    #[iden = "QuestionTemp"]
    Table,
}

#[derive(Iden, EnumIter)]
pub enum TopicTagTemp {
    #[iden = "TopicTagTemp"]
    Table,
}

#[derive(Iden, EnumIter)]
pub enum QuestionTopicTagTemp {
    #[iden = "QuestionTopicTagTemp"]
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        // Use `execute_unprepared` if the SQL statement doesn't have value bindings
        manager
            .create_table(
                Table::create()
                    .table(TopicTagTemp::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TopicTag::Name).string().not_null())
                    .col(ColumnDef::new(TopicTag::Id).string().primary_key())
                    .col(ColumnDef::new(TopicTag::Slug).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuestionTemp::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Question::AcRate).float().not_null())
                    .col(ColumnDef::new(Question::Difficulty).string().not_null())
                    .col(ColumnDef::new(Question::FreqBar).float())
                    .col(
                        ColumnDef::new(Question::FrontendQuestionId)
                            .string()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::IsFavor).integer().not_null())
                    .col(ColumnDef::new(Question::PaidOnly).integer().not_null())
                    .col(ColumnDef::new(Question::Status).string())
                    .col(ColumnDef::new(Question::Title).string().not_null())
                    .col(ColumnDef::new(Question::TitleSlug).string().not_null())
                    .col(ColumnDef::new(Question::HasSolution).integer().not_null())
                    .col(
                        ColumnDef::new(Question::HasVideoSolution)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuestionTopicTagTemp::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(QuestionTopicTag::QuestionId).string())
                    .col(ColumnDef::new(QuestionTopicTag::TagId).string())
                    .foreign_key(
                        &mut ForeignKey::create()
                            .from(QuestionTopicTagTemp::Table, QuestionTopicTag::QuestionId)
                            .to(QuestionTemp::Table, Question::FrontendQuestionId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .to(TopicTagTemp::Table, TopicTag::Id)
                            .from(QuestionTopicTagTemp::Table, QuestionTopicTag::TagId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .primary_key(
                        &mut Index::create()
                            .col(QuestionTopicTag::TagId)
                            .col(QuestionTopicTag::QuestionId)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            "
                INSERT INTO QuestionTemp
                SELECT * FROM `Question`;

                INSERT INTO TopicTagTemp
                SELECT * FROM `TopicTag`;

                INSERT INTO QuestionTopicTagTemp
                SELECT * FROM `QuestionTopicTag`;
            ",
        )
        .await?;

        manager
            .drop_table(Table::drop().table(QuestionTopicTag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TopicTag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await?;

        manager
            .rename_table(
                Table::rename()
                    .table(QuestionTemp::Table, Question::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .rename_table(
                Table::rename()
                    .table(TopicTagTemp::Table, TopicTag::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .rename_table(
                Table::rename()
                    .table(QuestionTopicTagTemp::Table, QuestionTopicTag::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
