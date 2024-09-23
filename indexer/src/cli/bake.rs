use std::ops::{Add, AddAssign, SubAssign};
use std::sync::{Arc, atomic};
use crate::tasks;
use std::sync::atomic::AtomicBool;
use log::{error, info};
use service::config::ConfigService;
use lib::error::Error;
use error_stack::{Report, Result, ResultExt};
use futures_util::future::try_join_all;
use tokio::time::sleep;
use service::store::service::StoreService;
use crate::LOG_TARGET;
use migration::{Migrator, MigratorTrait};
use service::cache::service::CacheService;
use service::event_queue::service::EventQueueService;
use service::message_broker::service::MessageBrokerService;
use service::services::Services;

#[tokio::main]
pub async fn start(config: ConfigService, product: String) -> Result<(), Error> {
    info!("Starting to bake...");

    // Listen for Ctrl+C and set the shutdown flag
    let shutdown = Arc::new(AtomicBool::new(false));
    tokio::task::spawn({
        let shutdown = shutdown.clone();
        async move {
            info!(target: LOG_TARGET, "Press Ctrl+C to exit");
            tokio::signal::ctrl_c().await.unwrap();
            shutdown.store(true, atomic::Ordering::Release);
            info!(target: LOG_TARGET, "Ctrl-C received, shutting down");
        }
    });

    let store = StoreService::new(config.database.clone()).await?;

    // Execute migrations
    info!(target: LOG_TARGET, "Executing migrations");
    if let Err(e) = Migrator::up(store.write(), None).await {
        error!(target: LOG_TARGET, "Failed to execute migration(s): {e:#?}");
        return Err(Report::new(Error::StoreMigration));
    }
    
    let services = Services {
        config: config.clone(),
        store: store.clone(),
        message_broker: MessageBrokerService::new(LOG_TARGET, config.redis.clone()).await?,
        cache: CacheService::new(config.redis.clone())?,
        event_queue: EventQueueService::new(store.clone()),
    };
    
    let mut tasks_to_join = vec![];
    
    // start baking
    let bake_task: tokio::task::JoinHandle<Result<(), Error>>  = tokio::task::spawn({
        async move {
            loop {
                if shutdown.load(atomic::Ordering::Acquire) {
                    break;
                }

            }
            tasks::
            Ok(())
        }
    });
    
    tasks_to_join.push(bake_task);
    tasks_to_join.push(
        services
            .event_queue
            .handle_events(services.clone(), shutdown.clone())
    );

    let results = try_join_all(tasks_to_join).await.change_context(Error::Unknown).unwrap();
    for result in results {
        result?;
    }

    Ok(())
}