use ethers::types::{Address, Bytes, U256};
// use serde_json::error::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("CLI command is unknown")]
    UnknownCLICommand,

    #[error("Stream error")]
    Stream,

    #[error("Failed to start chain")]
    ChainStart,

    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),

    #[error("Invalid configuration file format")]
    ConfigInvalid,

    #[error("Store error")]
    Store,

    #[error("Store is not available")]
    StoreNotAvailable,

    #[error("Failed to migrate store")]
    StoreMigration,

    #[error("Failed to create store collection")]
    FailedCreateStoreCollection,

    #[error("Failed to create store index")]
    FailedCreateStoreIndex,

    #[error("Entity already exists")]
    AlreadyExists,

    #[error("Failed to parse event log")]
    FailedToParseEventLog,

    #[error("Undefined log parameter")]
    UndefinedLogParam,

    #[error("Error during contract query")]
    ContractQuery,

    #[error("Contract doesn't have event")]
    ContractEventNotExist,

    #[error("Error occurred while processing event")]
    EventsRunner,

    #[error("GraphQL mutation failed")]
    GraphQLMutation,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Failed to serialize provided object")]
    SerdeSerialize,

    #[error("Redis Error")]
    Redis,

    #[error("Failed to connect to redis")]
    RedisConnect,

    #[error("AWS S3 error")]
    AWSS3,

    #[error("Resource not found")]
    NotFound,

    #[error("Tenderly request failed")]
    TenderlyRequestFailed,

    #[error("Tenderly invalid response")]
    TenderlyInvalidResponse,

    #[error("Discord error")]
    Discord,

    #[error("Reqwest error")]
    Reqwest,

    #[error("Unknown error")]
    Unknown,
}

pub struct ContractCallError {
    pub from: Address,
    pub to: Address,
    pub data: Bytes,
    pub value: U256,
    pub gas: Option<U256>,
    pub gas_price: Option<U256>,
    pub message: Option<String>,
}
