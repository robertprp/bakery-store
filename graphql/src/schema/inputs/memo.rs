use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use entity::vault_action::{ExecutorAction, ExecutorIntegration};
use serde::Serialize;

#[derive(Clone, Debug, Serialize, InputObject)]
pub struct AddTradeMemoInput {
    pub message: String,
    pub vault_uid: String,
    pub block_number: u64,
    pub integration: ExecutorIntegration,
    pub action: ExecutorAction,
    pub chain: String,
}

#[derive(Clone, Debug, Serialize, InputObject)]
pub struct GetTradingMemoInput {
    pub vault_uid: String,
    pub from_date: DateTime<Utc>,
    pub to_date: DateTime<Utc>,
}
