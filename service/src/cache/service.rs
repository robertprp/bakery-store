use std::ops::Deref;

use error_stack::{IntoReport, Result, ResultExt};
use lib::error::Error;
use redis::aio::Connection;
use redis::Client;

use crate::config::service::RedisConfig;

#[derive(Clone)]
pub struct CacheService(Client);

impl CacheService {
    pub fn new(config: RedisConfig) -> Result<Self, Error> {
        let client = Client::open(config.url)
            .into_report()
            .change_context(Error::Redis)?;

        Ok(CacheService(client))
    }

    pub async fn get_connection(&self) -> Result<Connection, Error> {
        self.0
            .get_tokio_connection()
            .await
            .into_report()
            .change_context(Error::RedisConnect)
    }
}

impl Deref for CacheService {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
