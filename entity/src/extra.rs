use async_graphql::Enum;
use sea_orm::{ ColumnTypeTrait, DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum EventMessageStatus {
    #[sea_orm(string_value = "PENDING")]
    Pending,
    #[sea_orm(string_value = "PROCESSED")]
    Processed,
    #[sea_orm(string_value = "FAILED")]
    Failed,
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum EventMessageType {
    #[sea_orm(string_value = "ORDER_CREATED")]
    OrderCreated,
    #[sea_orm(string_value = "ORDER_UPDATED")]
    OrderUpdated,
    #[sea_orm(string_value = "ORDER_DELETED")]
    OrderDeleted,
    #[sea_orm(string_value = "PRODUCT_CREATED")]
    ProductCreated,
    #[sea_orm(string_value = "PRODUCT_UPDATED")]
    ProductUpdated,
    #[sea_orm(string_value = "PRODUCT_DELETED")]
    ProductDeleted,
    #[sea_orm(string_value = "PRODUCT_STOCK_UPDATED")]
    ProductStockUpdated,
}