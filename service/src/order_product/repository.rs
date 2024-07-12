use error_stack::ResultExt;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryOrder};
use uuid::Uuid;
use crate::store::service::StoreService;
use entity::{order_product};
use lib::entity::{opt_to_active_value, opt_to_active_value_opt};
use lib::error::Error;
use crate::order_product::dto::*;

pub struct OrderProductRepository(StoreService);

impl OrderProductRepository {
    pub fn new(store: StoreService) -> Self {
        Self(store)
    }

    pub async fn create(&self, dto: CreateOrderProductDTO) -> error_stack::Result<order_product::Model, Error> {
        let order = order_product::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            order_id: ActiveValue::Set(dto.order_id),
            product_id: ActiveValue::Set(dto.product_id),
            quantity: ActiveValue::Set(dto.quantity),
            total_price: ActiveValue::Set(dto.total_price),
            ..Default::default()
        };

        let model = order.insert(self.store().write()).await.change_context(Error::Store)?;

        // should broadcast created model
        Ok(model)
    }

    pub async fn update(&self, dto: UpdateOrderProductDTO) -> error_stack::Result<order_product::Model, Error> {
        let order = order_product::ActiveModel {
            order_id: opt_to_active_value(dto.order_id),
            product_id: opt_to_active_value(dto.product_id),
            quantity: opt_to_active_value(dto.quantity),
            total_price: opt_to_active_value(dto.total_price),
            ..Default::default()
        };
        let model = order.update(self.store().write()).await.change_context(Error::Store)?;

        // should broadcast updated model
        Ok(model)
    }

}