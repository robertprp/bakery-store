use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bakery::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bakery::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bakery::Name).string().not_null())
                    .col(
                        ColumnDef::new(Bakery::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Bakery::UpdatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Bakery::ActiveAt).timestamp())
                    .col(ColumnDef::new(Bakery::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        manager.create_table(
            Table::create()
                .table(Order::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Order::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Order::Name).string().not_null())
                .col(
                    ColumnDef::new(Order::ProductId)
                        .uuid()
                        .not_null()
                )
                .col(
                    ColumnDef::new(Order::BakeryId)
                        .integer()
                        .not_null()
                )
                .col(
                    ColumnDef::new(Order::CreatedAt)
                        .timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Order::UpdatedAt)
                        .timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .col(ColumnDef::new(Order::ActiveAt).timestamp())
                .col(ColumnDef::new(Order::DeletedAt).timestamp())
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(Product::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Product::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Product::Name).string().not_null())
                .col(ColumnDef::new(Product::Price).decimal().not_null())
                .col(
                    ColumnDef::new(Product::CreatedAt)
                        .timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Product::UpdatedAt)
                        .timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .col(ColumnDef::new(Product::ActiveAt).timestamp())
                .col(ColumnDef::new(Product::DeletedAt).timestamp())
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .table(Order::Table)
                .name("order_product_id_idx")
                .col(Order::ProductId)
                .unique()
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .table(Order::Table)
                .name("order_bakery_id_idx")
                .col(Order::BakeryId)
                .unique()
                .to_owned(),
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Product::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(Order::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(Bakery::Table).if_exists().to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Bakery {
    Table,
    Id,
    Name,
    UpdatedAt,
    CreatedAt,
    DeletedAt,
    ActiveAt,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    Name,
    ProductId,
    BakeryId,
    UpdatedAt,
    CreatedAt,
    DeletedAt,
    ActiveAt,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    Price,
    UpdatedAt,
    CreatedAt,
    DeletedAt,
    ActiveAt,
}
