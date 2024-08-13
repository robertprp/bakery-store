use std::sync::Arc;
use chrono::Utc;
use colorful::Colorful;
use error_stack::ResultExt;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryOrder};
use crate::store::service::StoreService;
use entity::{bakery};
use lib::entity::{opt_to_active_value, opt_to_active_value_opt};
use lib::error::Error;
use crate::bakery::dto::{CreateBakeryDTO, UpdateBakeryDTO};

#[derive(Clone)]
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

    pub async fn update(&self, dto: UpdateBakeryDTO, db_tx: &DatabaseTransaction) -> error_stack::Result<bakery::Model, Error> {
        let bakery = bakery::ActiveModel {
            name: opt_to_active_value(dto.name),
            ..Default::default()
        };

        let model = bakery.update(db_tx).await.change_context(Error::Store)?;

        // should broadcast updated model
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
            .all(self.0.read())
            .await
            .change_context(Error::Store)
    }

    pub async fn find_all_deleted(&self) -> error_stack::Result<Vec<bakery::Model>, Error> {
        bakery::Entity::find()
            .filter(bakery::Column::DeletedAt.ne(None))
            .order_by_desc(bakery::Column::UpdatedAt)
            .all(self.0.read())
            .await
            .change_context(Error::Store)
    }

    pub fn reverse(x: i32) -> i32 {
        let is_negative = x.is_negative();


        let reverse = if is_negative {
            let mut negative_str = x.to_string().chars().skip(1).map(|c| c.to_string()).collect::<Vec<String>>()
            negative_str.reverse();
            String::from("-") + negative_str.join("").as_str()
        } else {
            x.to_string().chars().rev().collect::<String>()
        };

        let removed_zeros = reverse.trim_start_matches('0').to_string();

        if is_negative {
            removed_zeros.parse::<i32>().unwrap() * -1
        } else {
            removed_zeros.parse::<i32>().unwrap()
        }
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> error_stack::Result<Option<bakery::Model>, Error> {
        bakery::Entity::find()
            .filter(bakery::Column::Id.eq(id))
            .one(self.0.read())
            .await
            .change_context(Error::Store)
    }
}