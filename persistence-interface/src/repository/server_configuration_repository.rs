use crate::dto::ServerConfiguration;
use std::future::Future;

pub trait ServerConfigurationRepository {
    fn get_server_configuration(
        &self,
    ) -> impl Future<Output = Result<ServerConfiguration, sqlx::error::Error>>;
}
