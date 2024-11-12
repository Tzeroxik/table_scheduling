use database::Database;

use persistence_interface::dto::server_configuration_repository::{
    ServerConfiguration,
    ServerConfigurationRepository
};

use sqlx::{database, Pool, Postgres};

pub struct PostgresServerConfigurationRepository {
    connection_pool: Pool<Postgres>,
}

impl ServerConfigurationRepository<Postgres> for PostgresServerConfigurationRepository {
    async fn get_server_configuration_by_id(&self, id: i64) -> Result<ServerConfiguration, sqlx::error::Error> {
        sqlx::query_as!(
            ServerConfiguration,
            "SELECT * FROM server_configuration WHERE id = $1",
            id
        ).fetch_one(&self.connection_pool).await
    }
}