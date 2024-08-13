use lib::error::Error;
use crate::bakery::dto::{CreateBakeryDTO, UpdateBakeryDTO};
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

    pub async fn create(&self, bakery: CreateBakeryDTO) -> error_stack::Result<entity::bakery::Model, Error> {
        let db_tx = self.services.store.begin_transaction().await?;
        let bakery = self.repository.create(bakery, &db_tx).await?;

        self.services.store.commit_transaction(db_tx).await?;

        Ok(bakery)
    }

    pub async fn find_all(&self) -> error_stack::Result<Vec<entity::bakery::Model>, Error> {
        self.repository.find_all().await
    }

    pub async fn find_all_deleted(&self) -> error_stack::Result<Vec<entity::bakery::Model>, Error> {
        self.repository.find_all_deleted().await
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> error_stack::Result<Option<entity::bakery::Model>, Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn mark_as_deleted(&self, id: uuid::Uuid) -> error_stack::Result<entity::bakery::Model, Error> {
        let db_tx = self.services.store.begin_transaction().await?;
        let bakery = self.repository.mark_as_deleted(id, &db_tx).await?;

        self.services.store.commit_transaction(db_tx).await?;

        Ok(bakery)
    }

    pub async fn update(&self, bakery: UpdateBakeryDTO) -> error_stack::Result<entity::bakery::Model, Error> {
        let db_tx = self.services.store.begin_transaction().await?;
        let bakery = self.repository.update(bakery, &db_tx).await?;

        self.services.store.commit_transaction(db_tx).await?;

        Ok(bakery)
    }
}