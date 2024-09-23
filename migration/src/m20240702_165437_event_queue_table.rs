use sea_orm_migration::prelude::*;
use entity::extra::EventMessageStatus;

#[derive(DeriveMigrationName)]
pub struct Migration;

// Default Event Queue Status is PENDING
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EventMessage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EventMessage::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(EventMessage::EventType).string().not_null())
                    .col(
                        ColumnDef::new(EventMessage::Status)
                            .string()
                            .not_null()
                            .default(EventMessageStatus::Pending),
                    )
                    .col(ColumnDef::new(EventMessage::Payload).json().not_null())
                    .col(
                        ColumnDef::new(EventMessage::CreatedAt)
                            .date_time()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EventMessage::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum EventMessage {
    Table,
    Id,
    EventType,
    Status,
    Payload,
    CreatedAt,
}
