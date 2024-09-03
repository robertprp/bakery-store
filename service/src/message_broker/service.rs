use std::sync::Arc;
use error_stack::{IntoReport, Report, ResultExt, Result, FutureExt};
use futures_util::StreamExt;
use log::{info, warn};
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client, Msg};
use serde::Serialize;
use tokio_stream::wrappers::BroadcastStream;
use entity::{bakery, order, product, stock};
use lib::error::Error;
use crate::config::redis::RedisConfig;
use tokio::sync::broadcast::Receiver;

#[derive(Clone, Debug)]
pub enum Event {
    BakeryCreated(bakery::Model),
    BakeryUpdated(bakery::Model),
    BakeryDeleted(bakery::Model),
    ProductCreated(product::Model),
    ProductUpdated(product::Model),
    ProductDeleted(product::Model),
    OrderCreated(order::Model),
    OrderUpdated(order::Model),
    OrderDeleted(order::Model),
    StockCreated(stock::Model),
    StockUpdated(stock::Model),
}
pub struct MessageBrokerServiceInner {
    connection: ConnectionManager,
    channel: Receiver<Event>
}
#[derive(Clone)]
pub struct MessageBrokerService(Arc<MessageBrokerServiceInner>);

impl MessageBrokerService {
    pub async fn new(log_target: &str, config: RedisConfig) -> Result<MessageBrokerService, Error> {
        info!(target: log_target, "Connecting to redis at {}", config.url);

        let client = Client::open(config.url.clone())
            .change_context(Error::Redis).unwrap();

        let connection_manager = client.get_connection_manager().await.unwrap();

        let (tx, mut rx) = tokio::sync::broadcast::channel::<Event>(10_000);

        let mut pubsub = client.get_async_pubsub().await.unwrap();

        pubsub.subscribe("bakery").await.unwrap();
        pubsub.subscribe("product").await.unwrap();
        pubsub.subscribe("order").await.unwrap();
        pubsub.subscribe("stock").await.unwrap();

        tokio::spawn(async move {
            let mut stream = pubsub.into_on_message();

            while let Some(msg) = stream.next().await {
                let event = Self::parse_event_message(&msg, msg.get_channel_name().to_string());
                if let Err(e) = tx.send(event.unwrap()) {
                    warn!("Failed to send event: {e:?}");
                    continue;
                }
            }
        });

        Ok(MessageBrokerService(
            Arc::new(MessageBrokerServiceInner {
                connection: connection_manager,
                channel: rx
            })
        ))
    }

    pub fn parse_event_message(msg: &Msg, channel_name: String) -> error_stack::Result<Event, Error> {
        let event_payload = msg.get_payload::<String>();
        if let Err(e) = event_payload {
            warn!("Failed to get payload: {e:?}");
            return Err(Report::new(Error::Redis)).attach_printable("Failed to get payload");
        }

        let payload = event_payload.as_ref().unwrap();

        let event = match channel_name.as_str() {
            "bakery" => {
                match serde_json::from_str::<bakery::Model>(payload) {
                    Ok(bakery) => Event::BakeryCreated(bakery),
                    Err(e) => {
                        warn!("Failed to parse bakery event: {e:?}");
                        return Err(Report::new(Error::Redis)).attach_printable("Failed to parse bakery event");
                    }
                }
            }
            "product" => {
                match serde_json::from_str::<product::Model>(payload) {
                    Ok(product) => Event::ProductCreated(product),
                    Err(e) => {
                        warn!("Failed to parse product event: {e:?}");
                        return Err(Report::new(Error::Redis)).attach_printable("Failed to parse product event");
                    }
                }
            }
            "order" => {
                match serde_json::from_str::<order::Model>(payload) {
                    Ok(order) => Event::OrderCreated(order),
                    Err(e) => {
                        warn!("Failed to parse order event: {e:?}");
                        return Err(Report::new(Error::Redis)).attach_printable("Failed to parse order event");
                    }
                }
            }
            "stock" => {
                match serde_json::from_str::<stock::Model>(payload) {
                    Ok(stock) => Event::StockCreated(stock),
                    Err(e) => {
                        warn!("Failed to parse stock event: {e:?}");
                        return Err(Report::new(Error::Redis)).attach_printable("Failed to parse stock event");
                    }
                }
            }
            _ => {
                warn!("Unknown channel name: {channel_name}");
                return Err(Report::new(Error::Redis)).attach_printable("Unknown channel name");
            }
        };

        Ok(event)
    }

    pub async fn send<T>(&self, channel: String, msg: T) -> Result<(), Error>
    where T: Serialize
    {
        let mut connection = self.0.connection.clone();

        let msg = serde_json::to_string(&msg).unwrap();

        connection.publish(channel, msg).await.change_context(Error::Redis)?;

        Ok(())
    }

    pub async fn subscribe(&self) -> impl futures::Stream<Item = Event> {
        let stream = BroadcastStream::from(self.0.channel.resubscribe());

        stream.filter_map(|event| {
            futures::future::ready(match event {
                Ok(event) => Some(event),
                _ => None,
            })
        })
    }
}