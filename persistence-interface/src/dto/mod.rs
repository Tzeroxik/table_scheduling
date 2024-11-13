use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfiguration {
    pub id: i64,
    pub name: String,
    pub port: String,
}