use error_stack::{IntoReport, Result, ResultExt};
use lib::error::Error;
use rand::seq::SliceRandom;
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};
use crate::config::database::DatabaseConfig;

#[derive(Clone)]
pub struct StoreService {
    read_write: Vec<DatabaseConnection>,
    read_only: Vec<DatabaseConnection>,
}

impl StoreService {
    pub async fn new(config: DatabaseConfig) -> Result<Self, Error> {
        let mut read_only: Vec<DatabaseConnection> = Vec::new();
        let mut read_write: Vec<DatabaseConnection> = Vec::new();

        for server in config.servers {
            let mut options = ConnectOptions::new(server.url);
            options
                .sqlx_logging(true)
                .sqlx_logging_level(log::LevelFilter::Info);

            let db = Database::connect(options)
                .await
                .change_context(Error::Store)?;

            if !server.read_only {
                read_write.push(db.clone());
            }

            read_only.push(db);
        }

        Ok(StoreService {
            read_only,
            read_write,
        })
    }

    pub fn read(&self) -> &DatabaseConnection {
        self.read_only.choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn write(&self) -> &DatabaseConnection {
        self.read_write.choose(&mut rand::thread_rng()).unwrap()
    }

    /// Create a new database transaction
    pub async fn begin_transaction(&self) -> Result<DatabaseTransaction, Error> {
        self.write()
            .begin()
            .await
            .change_context(Error::Store)
    }

    /// Commit provided transaction
    pub async fn commit_transaction(&self, tx: DatabaseTransaction) -> Result<(), Error> {
        tx.commit().await.change_context(Error::Store)
    }

    /// Rollback provided transaction
    pub async fn rollback_transaction(&self, tx: DatabaseTransaction) -> Result<(), Error> {
        tx.rollback()
            .await
            .change_context(Error::Store)
    }
}
