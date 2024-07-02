use entity::event_message;
use error_stack::{Result, ResultExt};
use lib::error::Error;
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};
use entity::event_message::EventMessageStatus;
use crate::store::service::StoreService;

pub struct EventQueueRepository {
    store: StoreService,
}

impl EventQueueRepository {
    pub fn new(store: StoreService) -> Self {
        Self { store }
    }

    pub async fn get_pending_messages(&self) -> Result<Vec<event_message::Model>, Error> {
        event_message::Entity::find()
            .filter(event_message::Column::Status.eq(EventMessageStatus::Pending))
            .order_by(event_message::Column::CreatedAt, Order::Asc)
            .all(self.store.read())
            .await
            .change_context(Error::Unknown)
    }

    pub async fn get_failed_messages(&self) -> Result<Vec<event_message::Model>, Error> {
        event_message::Entity::find()
            .filter(event_message::Column::Status.eq(EventMessageStatus::Failed))
            .order_by(event_message::Column::CreatedAt, Order::Asc)
            .all(self.store.read())
            .await
            .change_context(Error::Unknown)
    }
}