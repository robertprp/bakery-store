use error_stack::ResultExt;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryOrder};
use uuid::Uuid;
use crate::store::service::StoreService;
use entity::{order};
use lib::entity::{opt_to_active_value, opt_to_active_value_opt};
use lib::error::Error;
use crate::order::dto::{CreateOrderDTO, UpdateOrderDTO};

pub struct OrderRepository(StoreService);

impl OrderRepository {
    pub fn new(store: StoreService) -> Self {
        Self(store)
    }

    pub async fn create(&self, dto: CreateOrderDTO) -> error_stack::Result<order::Model, Error> {
        let bakery_id = dto.bakery_id;

        let order = order::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            price: ActiveValue::Set(dto.price),
            bakery_id: ActiveValue::Set(bakery_id),
            ..Default::default()
        };

        let model = order.insert(self.0.write()).await.change_context(Error::Store)?;

        // should broadcast created model
        Ok(model)
    }

    pub async fn update(&self, dto: UpdateOrderDTO) -> error_stack::Result<order::Model, Error> {
        let order = order::ActiveModel {
            bakery_id: opt_to_active_value(dto.bakery_id),
            price: opt_to_active_value(dto.price),
            ..Default::default()
        };
        let model = order.update(self.0.write()).await.change_context(Error::Store)?;

        // should broadcast updated model
        Ok(model)
    }

    pub async fn find_all(&self) -> error_stack::Result<Vec<order::Model>, Error> {
        order::Entity::find()
            .order_by_desc(order::Column::UpdatedAt)
            .all(self.0.read())
            .await
            .change_context(Error::Store)
    }

    pub async fn find_all_active(&self) -> error_stack::Result<Vec<order::Model>, Error> {
        order::Entity::find()
            .filter(order::Column::DeletedAt.lt(chrono::Utc::now().naive_utc()))
            .order_by_desc(order::Column::UpdatedAt)
            .all(self.0.read())
            .await
            .change_context(Error::Store)
    }

    pub async fn find_by_bakery_id(&self, bakery_id: Uuid) -> error_stack::Result<Vec<order::Model>, Error> {
        order::Entity::find()
            .filter(order::Column::BakeryId.eq(bakery_id))
            .all(self.0.read())
            .await
            .change_context(Error::Store)
    }

    pub async fn mark_as_deleted(&self, order: order::Model) -> error_stack::Result<order::Model, Error> {
        let mut active_model = order::ActiveModel::from(order);
        active_model.deleted_at = opt_to_active_value_opt(Some(chrono::Utc::now().naive_utc()));
        let model = active_model.update(self.0.write()).await.change_context(Error::Store)?;

        // should broadcast updated model
        Ok(model)
    }



}