use async_graphql::Object;
use rust_decimal::Decimal;
use uuid::Uuid;
use entity::order;
use entity::order::Model;
use crate::helpers::date::DateTimeHelper;

#[derive(Clone)]
pub struct OrderType(entity::order::Model);

impl From<order::Model> for OrderType {
    fn from(value: Model) -> Self {
        OrderType(value)
    }
}

#[Object]
impl OrderType {
    async fn id(&self) -> String {
        format!("{:#x}", self.0.id)
    }

    async fn bakery_id(&self) -> &str {
        self.0.bakery_id.to_string().as_str()
    }

    async fn price(&self) -> Decimal {
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
}