use std::sync::Arc;
use crate::cache::service::CacheService;
use crate::config::ConfigService;
use crate::store::service::StoreService;
use crate::event_queue::service::EventQueueService;
use crate::message_broker::service::MessageBrokerService;

#[derive(Clone)]
pub struct Services {
    pub config: ConfigService,
    pub cache: CacheService,
    pub store: StoreService,
    pub event_queue: EventQueueService,
    pub message_broker: MessageBrokerService
}
