use async_graphql::Object;

pub struct OrderType(entity::order::Model);

#[Object]
impl OrderType {
    async fn id(&self) -> String {
        format!("{:#x}", self.0.id)
    }

    async fn bakery_id(&self) -> String {
        format!("{:#x}", self.0.bakery_id)
    }

    async fn product_id(&self) -> String {
        format!("{:#x}", self.0.product_id)
    }

    async fn quantity(&self) -> i32 {
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