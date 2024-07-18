use async_graphql::Object;
use crate::helpers::date::DateTimeHelper;

#[derive(Clone)]
struct BakeryType(pub entity::bakery::Model);

#[Object]
impl BakeryType {
    async fn id(&self) -> String {
        format!("{:#x}", self.0.id)
    }

    async fn name(&self) -> &str {
        &self.0.name
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