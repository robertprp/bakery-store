use std::fmt::Display;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use error_stack::{IntoReport, Report, ResultExt, Result};
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

impl Default for ConfigService {
    fn default() -> Self {
        Self::from_file(Path::new("./config.yaml")).unwrap()
    }
}
impl <'de> serde::Deserialize<'de> for ConfigService {
    fn deserialize<D>(deserialize: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Default)]
        struct AdHocConfig {
            pub database: DatabaseConfig,
            pub redis: RedisConfig,
            pub graphql: GQLConfig,
            pub jwt: JWTConfig
        }

        let ad_hoc: AdHocConfig = serde::Deserialize::deserialize(deserialize)?;

         let config_service = ConfigService::builder()
            .database(ad_hoc.database)
            .redis(ad_hoc.redis)
            .graphql(ad_hoc.graphql)
            .jwt(ad_hoc.jwt)
            .build()
             .map_err(|e| serde::de::Error::custom(e.to_string()));

        config_service
    }
}

impl FromStr for ConfigService {
    type Err = Report<Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // println!("{s:?}");

        // match serde_yaml::from_str(s) {
        //     Ok(config) => Ok(config),
        //     Err(e) => {
        //         println!("{e:?}");
        //         return Err(Report::new(Error::ConfigInvalid));
        //     }
        // }

        serde_yaml::from_str(s)
            .into_report()
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
            .into_report()
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