use serde::{Deserialize, Serialize};
use std::future::Future;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfiguration {
    pub id: i64,
    pub name: String,
    pub port: String,
    pub https: bool,
}

pub trait ServerConfigurationRepository {
    fn get_server_configuration(
        &self,
    ) -> impl Future<Output = Result<ServerConfiguration, sqlx::error::Error>>;
}
