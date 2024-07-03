use std::sync::Arc;
use error_stack::ResultExt;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryOrder};
use uuid::Uuid;
use crate::store::service::StoreService;
use entity::{order};
use lib::entity::{opt_to_active_value, opt_to_active_value_opt};
use lib::error::Error;
use crate::order::dto::CreateOrderDTO;

pub struct OrderRepository(StoreService);

impl OrderRepository {
    pub fn new(store: StoreService) -> Self {
        Self(store)
    }

    pub async fn create(&self, dto: CreateOrderDTO) -> error_stack::Result<order::Model, Error> {
        let now = chrono::Utc::now().naive_utc();
        let id = Uuid::new_v4();
        let name = dto.name;
        let product_id = dto.product_id;
        let bakery_id = dto.bakery_id;

        let order = order::ActiveModel {
            name: ActiveValue::Set(name),
            product_id: ActiveValue::Set(product_id),
            bakery_id: ActiveValue::Set(bakery_id),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            ..Default::default()
        };
        let model = dto.insert(self.store().write()).await.change_context(Error::Store)?;

        // should broadcast created model
        Ok(model)
    }
    pub async fn find_all(&self) -> error_stack::Result<Vec<order::Model>, Error> {
        order::Entity::find()
            .order_by_desc(order::Column::UpdatedAt)
            .all(self.store().read())
            .await
            .change_context(Error::Store)
    }

    pub async fn find_all_active(&self) -> error_stack::Result<Vec<order::Model>, Error> {
        order::Entity::find()
            .filter(order::Column::ActiveAt.is_not_null())
            .filter(order::Column::DeletedAt.lt(chrono::Utc::now().naive_utc()))
            .order_by_desc(order::Column::UpdatedAt)
            .all(self.store().read())
            .await
            .change_context(Error::Store)
    }

    pub async fn find_by_product_id(&self, product_id: Uuid) -> error_stack::Result<Vec<order::Model>, Error> {
        order::Entity::find()
            .filter(order::Column::ProductId.eq(product_id))
            .all(self.store().read())
            .await
            .change_context(Error::Store)
    }

    pub async fn mark_as_deleted(&self, order: order::Model) -> error_stack::Result<(), Error> {
        let mut active_model = order::ActiveModel::from(order);
        active_model.deleted_at = opt_to_active_value_opt(Some(chrono::Utc::now().naive_utc()));
        let model = active_model.update(self.store().write()).await.change_context(Error::Store)?;

        // should broadcast updated model
        Ok(())
    }



}