use error_stack::ResultExt;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use uuid::Uuid;
use entity::stock;
use lib::entity::opt_to_active_value;
use lib::error::Error;
use crate::stock::dto::{CreateStockDTO, UpdateStockDTO};
use crate::store::service::StoreService;

#[derive(Clone)]
pub struct StockRepository(StoreService);

impl StockRepository {
    pub fn new(store: StoreService) -> Self {
        Self(store)
    }

    pub async fn find_all(&self) -> error_stack::Result<Vec<stock::Model>, Error> {
        let stocks = stock::Entity::find().all(self.0.read()).await.change_context(Error::Store)?;
        Ok(stocks)
    }

    pub async fn find_by_id(&self, id: Uuid) -> error_stack::Result<Option<stock::Model>, Error> {
        let stock = stock::Entity::find_by_id(id)
            .one(self.0.read())
            .await
            .change_context(Error::Store)?;
        Ok(stock)
    }

    pub async fn create(&self, dto: CreateStockDTO) -> error_stack::Result<stock::Model, Error> {
        let stock = stock::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            product_id: ActiveValue::Set(dto.product_id),
            quantity: ActiveValue::Set(dto.quantity),
            ..Default::default()
        };

        let model = stock.insert(self.0.write()).await.change_context(Error::Store)?;

        // should broadcast created model
        Ok(model)
    }

    pub async fn update(&self, dto: UpdateStockDTO) -> error_stack::Result<stock::Model, Error> {
        let stock = stock::ActiveModel {
            product_id: opt_to_active_value(dto.product_id),
            quantity: opt_to_active_value(dto.quantity),
            ..Default::default()
        };
        let model = stock.update(self.0.write()).await.change_context(Error::Store)?;

        // should broadcast updated model
        Ok(model)
    }
}