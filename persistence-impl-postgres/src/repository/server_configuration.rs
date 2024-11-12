use persistence_interface::dto::server_configuration_repository::{
    ServerConfiguration,
    ServerConfigurationRepository,
};

use sqlx::{Pool, Postgres};

pub struct PostgresServerConfigurationRepository {
    connection_pool: Pool<Postgres>,
}

impl PostgresServerConfigurationRepository {
    pub fn from_connection_pool(connection_pool: Pool<Postgres>) -> impl ServerConfigurationRepository {
        PostgresServerConfigurationRepository { connection_pool }
    }
}

impl ServerConfigurationRepository for PostgresServerConfigurationRepository {
    async fn get_server_configuration(&self) -> Result<ServerConfiguration, sqlx::error::Error> {
        sqlx::query_as!(ServerConfiguration,"SELECT * FROM server_configuration WHERE id = 1")
            .fetch_one(&self.connection_pool)
            .await
    }
}