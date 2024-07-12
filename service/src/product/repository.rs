use error_stack::ResultExt;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use uuid::Uuid;
use entity::product;
use lib::entity::opt_to_active_value;
use lib::error::Error;
use crate::product::dto::{CreateProductDTO, UpdateProductDTO};
use crate::store::service::StoreService;

pub struct ProductRepository(StoreService);

impl ProductRepository {
    pub fn new(store: StoreService) -> Self {
        Self(store)
    }

    pub async fn find_all(&self) -> error_stack::Result<Vec<product::Model>, Error> {
        let products = product::Entity::find().all(self.store().read()).await.change_context(Error::Store)?;
        Ok(products)
    }

    pub async fn find_by_id(&self, id: Uuid) -> error_stack::Result<Option<product::Model>, Error> {
        let product = product::Entity::find_by_id(id)
            .find_opt(self.store().read())
            .await
            .change_context(Error::Store)?;
        Ok(product)
    }


    pub async fn create(&self, dto: CreateProductDTO) -> error_stack::Result<product::Model, Error> {
        let product = product::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            name: ActiveValue::Set(dto.name),
            price: ActiveValue::Set(dto.price),
            ..Default::default()
        };

        let model = product.insert(self.store().write()).await.change_context(Error::Store)?;

        // should broadcast created model
        Ok(model)
    }

    pub async fn update(&self, dto: UpdateProductDTO) -> error_stack::Result<product::Model, Error> {
        let product = product::ActiveModel {
            name: opt_to_active_value(dto.name),
            price: opt_to_active_value(dto.price),
            ..Default::default()
        };
        let model = product.update(self.store().write()).await.change_context(Error::Store)?;

        // should broadcast updated model
        Ok(model)
    }

    pub async fn mark_deleted(&self, id: Uuid) -> error_stack::Result<product::Model, Error> {
        let product = product::ActiveModel {
            id: ActiveValue::Set(id),
            deleted_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default()
        };

        let model = product.update(self.store().write()).await.change_context(Error::Store)?;

        // should broadcast updated model
        Ok(model)
    }
}