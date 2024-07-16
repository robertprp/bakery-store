use std::fmt::Display;

use async_graphql::{Enum, InputObject};

use serde::Serialize;

#[derive(Clone, Debug, Serialize, InputObject)]
pub struct AssetDatasetInput {
    pub aggregate: AssetDatasetAggregation,
    pub from_symbol: String,
    pub to_symbol: String,
    pub from_time: i64,
    pub to_time: i64,
}

#[derive(Clone, Debug, Serialize, InputObject)]
pub struct AssetDatasetEventInput {
    pub aggregate: AssetDatasetAggregation,
    pub from_symbol: String,
    pub to_symbol: String,
}

#[derive(Clone, Debug, Serialize, InputObject)]
pub struct AssetPairInput {
    pub first_asset: String,
    pub second_asset: String,
    pub chain: AssetDatasetChain,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize)]
pub enum AssetDatasetAggregation {
    M1,
    M5,
    M15,
    H1,
    H4,
    D1,
}

impl ToString for AssetDatasetAggregation {
    fn to_string(&self) -> String {
        match self {
            AssetDatasetAggregation::M1 => "1min".to_string(),
            AssetDatasetAggregation::M5 => "5min".to_string(),
            AssetDatasetAggregation::M15 => "15min".to_string(),
            AssetDatasetAggregation::H1 => "1hour".to_string(),
            AssetDatasetAggregation::H4 => "4hour".to_string(),
            AssetDatasetAggregation::D1 => "1day".to_string(),
        }
    }
}

impl From<AssetDatasetAggregation> for i64 {
    fn from(value: AssetDatasetAggregation) -> Self {
        match value {
            AssetDatasetAggregation::M1 => 1,
            AssetDatasetAggregation::M5 => 5,
            AssetDatasetAggregation::M15 => 15,
            AssetDatasetAggregation::H1 => 60,
            AssetDatasetAggregation::H4 => 240,
            AssetDatasetAggregation::D1 => 1440,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize)]
pub enum AssetDatasetChain {
    Arbitrum,
    Optimism,
}

impl Display for AssetDatasetChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetDatasetChain::Arbitrum => write!(f, "Arbitrum"),
            AssetDatasetChain::Optimism => write!(f, "Optimism"),
        }
    }
}
