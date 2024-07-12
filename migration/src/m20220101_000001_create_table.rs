use sea_orm_migration::prelude::*;
use crate::ColumnType::Uuid;

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
                    .col(ColumnDef::new(Bakery::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await?;

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
                .col(ColumnDef::new(Product::DeletedAt).timestamp())
                .col(ColumnDef::new(Product::ActiveAt).timestamp())
                .to_owned(),
        ).await?;

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
                .col(ColumnDef::new(Order::Price).decimal().not_null())
                .col(ColumnDef::new(Order::BakeryId).uuid().not_null())
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
                .col(ColumnDef::new(Order::DeletedAt).timestamp())
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(OrderProduct::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(OrderProduct::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(OrderProduct::OrderId).uuid().not_null())
                .col(ColumnDef::new(OrderProduct::ProductId).uuid().not_null())
                .col(ColumnDef::new(OrderProduct::Quantity).decimal().not_null())
                .col(ColumnDef::new(OrderProduct::TotalPrice).decimal().not_null())
                .col(
                    ColumnDef::new(OrderProduct::CreatedAt)
                        .timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .col(
                    ColumnDef::new(OrderProduct::UpdatedAt)
                        .timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .col(ColumnDef::new(OrderProduct::DeletedAt).timestamp())
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(Stock::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Stock::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(Stock::ProductId)
                        .uuid()
                        .not_null()
                )
                .col(ColumnDef::new(Stock::Quantity).decimal().not_null())
                .col(
                    ColumnDef::new(Stock::CreatedAt)
                        .timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Stock::UpdatedAt)
                        .timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .col(ColumnDef::new(Stock::DeletedAt).timestamp())
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .table(OrderProduct::Table)
                .name("order_product_idx")
                .col(OrderProduct::ProductId)
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
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    Price,
    BakeryId,
    UpdatedAt,
    CreatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum OrderProduct {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity,
    TotalPrice,
    UpdatedAt,
    CreatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Stock {
    Table,
    Id,
    ProductId,
    Quantity,
    UpdatedAt,
    CreatedAt,
    DeletedAt,
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
