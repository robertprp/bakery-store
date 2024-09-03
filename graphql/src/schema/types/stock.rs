use async_graphql::Object;
use entity::stock;
use crate::helpers::date::DateTimeHelper;
use sea_orm::prelude::Decimal;

pub struct StockType(stock::Model);

impl From<stock::Model> for StockType {
    fn from(value: stock::Model) -> Self {
        StockType(value)
    }
}

#[Object]
impl StockType {
    async fn product_id(&self) -> &str {
        &self.0.product_id.
    }

    async fn quantity(&self) -> Decimal {
        self.0.quantity
    }

    async fn created_at(&self) -> String {
        DateTimeHelper::new(self.0.created_at).to_rfc3339()
    }

    async fn updated_at(&self) -> String {
        DateTimeHelper::new(self.0.updated_at).to_rfc3339()
    }

    async fn deleted_at(&self) -> Option<String> {
        if let Some(deleted_at) = self.0.deleted_at {
            Some(DateTimeHelper::new(deleted_at).to_rfc3339())
        } else {
            None
        }
    }
}