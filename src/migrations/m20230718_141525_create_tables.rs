use sea_orm::EnumIter;
use sea_orm_migration::prelude::*;

#[derive(Iden, EnumIter)]
pub enum TopicTag {
    #[iden = "TopicTag"]
    Table,
    #[iden = "name"]
    Name,
    #[iden = "id"]
    Id,
    #[iden = "slug"]
    Slug,
}

#[derive(Iden, EnumIter)]
pub enum Question {
    #[iden = "Question"]
    Table,
    #[iden = "ac_rate"]
    AcRate,
    #[iden = "difficulty"]
    Difficulty,
    #[iden = "freq_bar"]
    FreqBar,
    #[iden = "frontend_question_id"]
    FrontendQuestionId,
    #[iden = "is_favor"]
    IsFavor,
    #[iden = "paid_only"]
    PaidOnly,
    #[iden = "status"]
    Status,
    #[iden = "title"]
    Title,
    #[iden = "title_slug"]
    TitleSlug,
    #[iden = "has_solution"]
    HasSolution,
    #[iden = "has_video_solution"]
    HasVideoSolution,
}

#[derive(Iden, EnumIter)]
pub enum QuestionTopicTag {
    #[iden = "QuestionTopicTag"]
    Table,
    #[iden = "question_id"]
    QuestionId,
    #[iden = "tag_id"]
    TagId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TopicTag::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TopicTag::Name).string())
                    .col(ColumnDef::new(TopicTag::Id).string().primary_key())
                    .col(ColumnDef::new(TopicTag::Slug).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Question::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Question::AcRate).float())
                    .col(ColumnDef::new(Question::Difficulty).string())
                    .col(ColumnDef::new(Question::FreqBar).float())
                    .col(
                        ColumnDef::new(Question::FrontendQuestionId)
                            .string()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::IsFavor).integer())
                    .col(ColumnDef::new(Question::PaidOnly).integer())
                    .col(ColumnDef::new(Question::Status).string())
                    .col(ColumnDef::new(Question::Title).string())
                    .col(ColumnDef::new(Question::TitleSlug).string())
                    .col(ColumnDef::new(Question::HasSolution).integer())
                    .col(ColumnDef::new(Question::HasVideoSolution).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuestionTopicTag::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(QuestionTopicTag::QuestionId).string())
                    .col(ColumnDef::new(QuestionTopicTag::TagId).string())
                    .foreign_key(
                        &mut ForeignKey::create()
                            .from(QuestionTopicTag::Table, QuestionTopicTag::QuestionId)
                            .to(Question::Table, Question::FrontendQuestionId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .to(TopicTag::Table, TopicTag::Id)
                            .from(QuestionTopicTag::Table, QuestionTopicTag::TagId)
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
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TopicTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(QuestionTopicTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = Table::create()
            .table(TopicTag::Table)
            .if_not_exists()
            .col(ColumnDef::new(TopicTag::Name).string())
            .col(ColumnDef::new(TopicTag::Id).string().primary_key())
            .col(ColumnDef::new(TopicTag::Slug).string())
            .to_owned()
            .to_string(SqliteQueryBuilder);

        let t2 = Table::create()
            .table(Question::Table)
            .if_not_exists()
            .col(ColumnDef::new(Question::AcRate).float())
            .col(ColumnDef::new(Question::Difficulty).string())
            .col(ColumnDef::new(Question::FreqBar).float())
            .col(
                ColumnDef::new(Question::FrontendQuestionId)
                    .string()
                    .primary_key(),
            )
            .col(ColumnDef::new(Question::IsFavor).integer())
            .col(ColumnDef::new(Question::PaidOnly).integer())
            .col(ColumnDef::new(Question::Status).string())
            .col(ColumnDef::new(Question::Title).string())
            .col(ColumnDef::new(Question::TitleSlug).string())
            .col(ColumnDef::new(Question::HasSolution).integer())
            .col(ColumnDef::new(Question::HasVideoSolution).integer())
            .to_owned()
            .to_string(SqliteQueryBuilder);

        let t3 = Table::create()
            .table(QuestionTopicTag::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(QuestionTopicTag::QuestionId)
                    .string()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(QuestionTopicTag::TagId)
                    .string()
                    .primary_key(),
            )
            .foreign_key(
                &mut ForeignKey::create()
                    .from(QuestionTopicTag::Table, QuestionTopicTag::QuestionId)
                    .to(Question::Table, Question::FrontendQuestionId)
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .foreign_key(
                &mut ForeignKey::create()
                    .to(TopicTag::Table, TopicTag::Id)
                    .from(QuestionTopicTag::Table, QuestionTopicTag::TagId)
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .to_owned()
            .to_string(SqliteQueryBuilder);
        println!("{}\n{}\n{}", t, t2, t3);
    }
}
