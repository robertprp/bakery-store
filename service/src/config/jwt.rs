use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct JWTConfig {
    pub public_key: String,
    pub private_key: String
}