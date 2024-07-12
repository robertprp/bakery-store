use sea_orm::DatabaseTransaction;
use lib::error::Error;
use crate::bakery::dto::CreateBakeryDTO;
use crate::bakery::repository::BakeryRepository;
use crate::services::Services;

pub struct BakeryService {
    pub services: Services,
    pub repository: BakeryRepository,
}

impl BakeryService {
    pub fn new(services: Services, repository: BakeryRepository) -> Self {
        Self { services, repository }
    }

    pub async fn create(&self, bakery: CreateBakeryDTO, db_tx: &DatabaseTransaction) -> error_stack::Result<entity::bakery::Model, Error> {
        self.repository.create(bakery, db_tx).await
    }
}