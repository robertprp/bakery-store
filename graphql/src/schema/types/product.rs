use async_graphql::Object;
use rust_decimal::Decimal;
use crate::helpers::date::DateTimeHelper;

pub struct ProductType(entity::product::Model);

#[Object]
impl ProductType {
    async fn id(&self) -> String {
        format!("{:#x}", self.0.id)
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn price (&self) -> Decimal {
        self.0.price
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

    async fn active_at(&self) -> Option<String> {
        if let Some(active_at) = self.0.active_at {
            Some(DateTimeHelper::new(active_at).to_rfc3339())
        } else {
            None
        }
    }
}