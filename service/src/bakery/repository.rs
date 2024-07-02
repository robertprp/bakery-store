use std::sync::Arc;
use error_stack::ResultExt;
use sea_orm::{ColumnTrait, EntityTrait, QueryOrder};
use crate::store::service::StoreService;
use entity::{bakery};
use lib::error::Error;

pub struct BakeryRepository(StoreService);

impl BakeryRepository {
    pub fn new(store: StoreService) -> Self {
        Self(store)
    }

    pub async fn find_all(&self) -> error_stack::Result<Vec<bakery::Model>, Error> {
        bakery::Entity::find()
            .order_by_desc(bakery::Column::UpdatedAt)
            .all(self.store().read())
            .await
            .change_context(Error::Store)
    }

    pub async fn find_all_active(&self) -> error_stack::Result<Vec<bakery::Model>, Error> {
        bakery::Entity::find()
            .filter(bakery::Column::ActiveAt.is_not_null())
            .filter(bakery::Column::DeletedAt.lt(chrono::Utc::now().naive_utc()))
            .order_by_desc(bakery::Column::UpdatedAt)
            .all(self.store().read())
            .await
            .change_context(Error::Store)
    }
}