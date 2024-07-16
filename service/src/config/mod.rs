use std::fmt::Display;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use error_stack::{IntoReport, Report, ResultExt};
use serde::{Deserialize, Deserializer, Serialize};
use lib::error::Error;
use crate::config::database::DatabaseConfig;
use crate::config::gql::GQLConfig;
use crate::config::jwt::JWTConfig;
use crate::config::redis::RedisConfig;

pub mod database;
pub mod gql;
pub mod redis;
pub mod jwt;

#[derive(Debug, Serialize)]
pub struct ConfigServiceInner {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub graphql: GQLConfig,
    pub jwt: JWTConfig
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigService(Arc<ConfigServiceInner>);

impl <'de> Deserialize<'de> for ConfigService {
    fn deserialize<D>(deserialize: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Default)]
        struct AdHocConfig {
            pub database: DatabaseConfig,
            pub redis: RedisConfig,
            pub graphql: GQLConfig,
            pub jwt: JWTConfig
        }

        let ad_hoc: AdHocConfig = serde::Deserialize::deserialize(deserialize)?;

        ConfigService::builder()
            .database(ad_hoc.database)
            .redis(ad_hoc.redis)
            .graphql(ad_hoc.graphql)
            .build()
    }
}

impl FromStr for ConfigService {
    type Err = Report<Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err>{
        serde_yaml::from_str(s)
            .change_context(Error::ConfigInvalid)
    }
}
#[buildstructor::buildstructor]
impl ConfigService {
    #[builder]
    pub fn new(database: DatabaseConfig, redis: RedisConfig, graphql: GQLConfig, jwt: JWTConfig) -> Result<Self, Error> {
        let inner_config = ConfigServiceInner {
            database,
            redis,
            graphql,
            jwt,
        };

        Ok(ConfigService(Arc::new(inner_config)))
    }

    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let config = fs::read_to_string(path)
            .change_context(Error::ConfigNotFound(path.to_str().unwrap().to_string()))?;

        config.parse()
    }
}

impl Display for ConfigService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_yaml::to_string(&self).unwrap())
    }
}

impl Deref for ConfigService {
    type Target = ConfigServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ConfigService {
    pub fn inner(&self) -> Arc<ConfigServiceInner> {
        self.0.clone()
    }
}