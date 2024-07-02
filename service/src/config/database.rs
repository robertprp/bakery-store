use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct DatabaseConfigServer {
    pub url: String,
    pub read_only: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct DatabaseConfig {
    pub servers: Vec<DatabaseConfigServer>,
}