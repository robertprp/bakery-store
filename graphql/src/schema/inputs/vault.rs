use async_graphql::InputObject;
use entity::vault::{
    ManagerFocusTime, ManagerParticipation, ManagerSentiment, ManagerStrategy, MarketCapSegment,
    VaultTag,
};
use serde::Serialize;

#[derive(Clone, Debug, Serialize, InputObject)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVaultInput {
    /// UID of vault to update
    #[serde(skip_serializing)]
    pub uid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(validator(min_length = 1, max_length = 50))]
    pub slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(validator(min_length = 0, max_length = 120))]
    pub short_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(validator(min_length = 0, max_length = 1000))]
    pub full_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_sentiment: Option<ManagerSentiment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<VaultTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_focus_time: Option<ManagerFocusTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_participation: Option<ManagerParticipation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_strategy: Option<ManagerStrategy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[graphql(validator(list, min_length = 20, max_length = 100, max_items = 10))]
    pub asset_watchlist: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_cap_segment: Option<MarketCapSegment>,
}
