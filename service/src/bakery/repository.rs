use std::sync::Arc;
use error_stack::ResultExt;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryOrder};
use crate::store::service::StoreService;
use entity::{bakery};
use lib::entity::opt_to_active_value_opt;
use lib::error::Error;
use crate::bakery::dto::CreateBakeryDTO;

pub struct BakeryRepository(StoreService);

impl BakeryRepository {
    pub fn new(store: StoreService) -> Self {
        Self(store)
    }

    pub async fn create(&self, dto: CreateBakeryDTO, db_tx: &DatabaseTransaction) -> error_stack::Result<bakery::Model, Error> {
        let now = chrono::Utc::now().naive_utc();
        let active_at = Option::from(dto.active_at.unwrap().naive_utc());
        let bakery = bakery::ActiveModel {
            id: ActiveValue::Set(uuid::Uuid::new_v4()),
            name: ActiveValue::Set(dto.name),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            active_at: opt_to_active_value_opt(active_at),
            ..Default::default()
        };

        let model = bakery.insert(db_tx).await.change_context(Error::Store)?;

        // should broadcast created model
        Ok(model)
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