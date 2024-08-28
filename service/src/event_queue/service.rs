use std::sync::{Arc, atomic};
use std::sync::atomic::AtomicBool;
use chrono::Utc;
use error_stack::{FutureExt, IntoReport, Report, ResultExt};
use log::{info, warn};
use sea_orm::{ActiveEnum, ActiveModelTrait, ActiveValue, DatabaseTransaction};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use entity::event_message;
use entity::event_message::{EventMessageStatus, EventMessageType, Model};
use entity::prelude::EventMessage;
use lib::error::Error;
use crate::event_queue::repository::EventQueueRepository;
use crate::message_broker::service::Event;
use crate::services::Services;
use crate::store::service::StoreService;

#[derive(Clone)]
pub struct EventQueueService {
    store: StoreService,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    pub product: entity::product::Model,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderUpdatedEvent {
    pub order: entity::order::Model,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderDeletedEvent {
    pub order: entity::order::Model,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProductCreatedEvent {
    pub product: entity::product::Model,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ProductUpdatedEvent {
    pub product: entity::product::Model,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ProductDeletedEvent {
    pub product: entity::product::Model,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ProductStockUpdatedEvent {
    pub product: entity::product::Model,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum EventPayload {
    OrderCreated(OrderCreatedEvent),
    OrderUpdated(OrderUpdatedEvent),
    OrderDeleted(OrderDeletedEvent),
    ProductCreated(ProductCreatedEvent),
    ProductUpdated(ProductUpdatedEvent),
    ProductDeleted(ProductDeletedEvent),
    ProductStockUpdated(ProductStockUpdatedEvent),
}

impl From<EventPayload> for String {
    fn from(value: EventPayload) -> Self {
        match value {
            EventPayload::OrderCreated(_) => "OrderCreated".to_string(),
            EventPayload::OrderUpdated(_) => "OrderUpdated".to_string(),
            EventPayload::OrderDeleted(_) => "OrderDeleted".to_string(),
            EventPayload::ProductCreated(_) => "ProductCreated".to_string(),
            EventPayload::ProductUpdated(_) => "ProductUpdated".to_string(),
            EventPayload::ProductDeleted(_) => "ProductDeleted".to_string(),
            EventPayload::ProductStockUpdated(_) => "ProductStockUpdated".to_string(),
        }
    }
}

impl From<EventPayload> for EventMessageType {
    fn from(value: EventPayload) -> Self {
        match value {
            EventPayload::OrderCreated(_) => EventMessageType::OrderCreated,
            EventPayload::OrderUpdated(_) => EventMessageType::OrderUpdated,
            EventPayload::OrderDeleted(_) => EventMessageType::OrderDeleted,
            EventPayload::ProductCreated(_) => EventMessageType::ProductCreated,
            EventPayload::ProductUpdated(_) => EventMessageType::ProductUpdated,
            EventPayload::ProductDeleted(_) => EventMessageType::ProductDeleted,
            EventPayload::ProductStockUpdated(_) => EventMessageType::ProductStockUpdated,
        }
    }
}


impl EventQueueService {
    pub fn new(store: StoreService) -> Self {
        Self { store }
    }

    pub async fn send(
        &self,
        payload: EventPayload,
        db_tx: &DatabaseTransaction,
    ) -> Result<entity::event_message::Model, Error> {
        let payload_json = serde_json::to_value(payload.clone()).change_context(Error::Unknown)?;

        let message = event_message::ActiveModel {
            id: ActiveValue::Set(uuid::Uuid::new_v4()),
            status: ActiveValue::Set(EventMessageStatus::Pending),
            event_type: ActiveValue::Set(payload.clone().into()),
            payload: ActiveValue::Set(payload_json),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
        };

        let message = message
            .insert(db_tx)
            .await

            .change_context(Error::Unknown)?;

        Ok(message)
    }
    pub async fn update_status(
        &self,
        message: &Model,
        status: EventMessageStatus,
    ) -> Result<(), Error> {
        let mut message = event_message::ActiveModel::from(message.clone());
        message.status = ActiveValue::Set(status.into());

        message
            .update(self.store.write())
            .await
            .change_context(Error::Unknown)
            .attach_printable("Failed to update event message status")?;

        Ok(())
    }


}

pub async fn handle_events(services: Services, shutdown: Arc<AtomicBool>) -> JoinHandle<()> {
    info!("Starting event queue service");

    let handlers = tokio::task::spawn(
        async move {
            let repository = EventQueueRepository::new(services.store.clone());
            let event_queue_service = services.event_queue.clone();

            loop {
                if shutdown.load(atomic::Ordering::Acquire) {
                    break;
                }

                let messages = repository.get_pending_messages().await.unwrap();

                for message in messages {
                    let payload: Result<EventPayload, serde_json::error::Error> = serde_json::from_value(message.payload);

                    info!("Processing event: {:?}", String::from(payload.clone()));

                    if let Err(err) = payload {
                        warn!("Failed to deserialize event payload: {:?}", err);
                        event_queue_service.update_status(&message, EventMessageStatus::Failed).await.unwrap();
                        continue;
                    }

                    event_queue_service.update_status(&message, EventMessageStatus::Processed).await.unwrap();
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    );

    handlers
}
