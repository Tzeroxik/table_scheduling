use persistence_interface::migration::{Migration, MigrationError};
use persistence_interface::repository::server_configuration_repository::{ServerConfigurationRepository};

use sqlx::{query, query_as, Executor, PgPool};
use persistence_interface::DatabaseOperations;
use persistence_interface::dto::ServerConfiguration;

#[derive(Debug, Clone)]
pub struct PostgresDatabaseOperations {
    connection_pool: PgPool,
}

impl PostgresDatabaseOperations {
    pub async fn from_con_str(
        con_str: &str,
    ) -> Result<PostgresDatabaseOperations, sqlx::error::Error> {
        let pool = PgPool::connect(con_str).await?;
        Ok(PostgresDatabaseOperations::new(pool))
    }

    pub fn new(connection_pool: PgPool) -> PostgresDatabaseOperations {
        PostgresDatabaseOperations { connection_pool }
    }
}

impl Migration for PostgresDatabaseOperations {
    async fn migrate(&self, path: &str) -> Result<u64, MigrationError> {
        let query_str = match std::fs::read_to_string(path) {
            Ok(str) => str,
            Err(e) => return MigrationError::MigrationFileReadError(e).into(),
        };

        let query = query(&query_str);

        match self.connection_pool.execute(query).await {
            Ok(res) => Ok(res.rows_affected()),
            Err(e) => MigrationError::MigrationExecutionError(e).into(),
        }
    }
}

impl ServerConfigurationRepository for PostgresDatabaseOperations {
    async fn get_server_configuration(&self) -> Result<ServerConfiguration, sqlx::error::Error> {
        query_as!(
            ServerConfiguration,
            r#"SELECT * FROM server_configuration WHERE id = 1"#
        )
        .fetch_one(&self.connection_pool)
        .await
    }
}

impl DatabaseOperations for PostgresDatabaseOperations {}