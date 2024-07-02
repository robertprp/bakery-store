use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct GQLConfig {
    pub listen: String,
    pub endpoint: String,
    pub subscription_endpoint: String,
}