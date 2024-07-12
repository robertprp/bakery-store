use std::sync::Arc;
use chrono::Utc;
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
        let bakery = bakery::ActiveModel {
            id: ActiveValue::Set(uuid::Uuid::new_v4()),
            name: ActiveValue::Set(dto.name),
            ..Default::default()
        };

        let model = bakery.insert(db_tx).await.change_context(Error::Store)?;

        // should broadcast created model
        Ok(model)
    }

    pub async fn mark_as_deleted(&self, id: uuid::Uuid, db_tx: &DatabaseTransaction) -> error_stack::Result<bakery::Model, Error> {
        let model = bakery::ActiveModel {
            id: ActiveValue::Set(id),
            deleted_at: opt_to_active_value_opt(Some(Utc::now().naive_utc())),
            ..Default::default()
        };

        model.update(db_tx).await.change_context(Error::Store)
    }

    pub async fn find_all(&self) -> error_stack::Result<Vec<bakery::Model>, Error> {
        bakery::Entity::find()
            .filter(bakery::Column::DeletedAt.is_null())
            .order_by_desc(bakery::Column::UpdatedAt)
            .all(self.store().read())
            .await
            .change_context(Error::Store)
    }

    pub async fn find_all_deleted(&self) -> error_stack::Result<Vec<bakery::Model>, Error> {
        bakery::Entity::find()
            .filter(bakery::Column::DeletedAt.not_eq(None))
            .order_by_desc(bakery::Column::UpdatedAt)
            .all(self.store().read())
            .await
            .change_context(Error::Store)
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> error_stack::Result<Option<bakery::Model>, Error> {
        bakery::Entity::find()
            .filter(bakery::Column::Id.eq(id))
            .one(self.store().read())
            .await
            .change_context(Error::Store)
    }
}