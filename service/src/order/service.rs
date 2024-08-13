use sea_orm::DatabaseTransaction;
use lib::error::Error;
use crate::order::dto::CreateOrderDTO;
use crate::order::repository::OrderRepository;
use crate::order_product::repository::OrderProductRepository;
use crate::services::{ Services};

pub struct OrderService {
    pub services: Services,
}

impl OrderService {
    pub fn new(services: Services) -> Self {
        Self { services }
    }

    pub async fn create(&self, order: CreateOrderDTO, db_tx: &DatabaseTransaction) -> error_stack::Result<entity::order::Model, Error> {
        todo!()
    }
}