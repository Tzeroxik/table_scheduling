#[cfg(test)]
mod tests {
    use crate::connection;
    use persistence_impl_postgres::PostgresDatabaseOperations;
    use persistence_interface::dto::migration::Migration;
    use persistence_interface::dto::server_configuration::ServerConfigurationRepository;

    #[tokio::test]
    async fn test_get_server_configuration() {
        let pool = connection::get_connection_pool().await;

        let configuration: impl ServerConfigurationRepository =
            PostgresDatabaseOperations::new(pool)
                .get_server_configuration()
                .await
                .expect("Failed to get server configuration");

        println!("{configuration:?}");
    }

    #[tokio::test]
    async fn test_migration() {
        let pool = connection::get_connection_pool().await;

        let operations: impl Migration = PostgresDatabaseOperations::new(pool);

        let migration = operations.migrate("./sql/schema.sql").await;

        match migration {
            Err(e) => panic!("{e:?}"),
            Ok(rows_affected) => assert_eq!(rows_affected, 0u64),
        }
    }
}

mod connection {
    use crate::configuration;
    use config::FileFormat;
    use sqlx::PgPool;

    pub async fn get_connection_pool() -> PgPool {
        let settings =
            configuration::load_configurations("./tests/database.yaml", FileFormat::Yaml)
                .expect("Failed to load ./tests/database.yaml");

        PgPool::connect(&settings.connection_string())
            .await
            .expect("Failed to connect to Postgres")
    }
}
mod configuration {
    use config::{Config, File, FileFormat, FileSourceFile};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct DatabaseSettings {
        pub db_type: String,
        pub username: String,
        pub password: String,
        pub host: String,
        pub port: u16,
        pub database_name: String,
    }

    impl DatabaseSettings {
        pub fn connection_string(&self) -> String {
            format!(
                "{}://{}:{}@{}:{}/{}",
                self.db_type,
                self.username,
                self.password,
                self.host,
                self.port,
                self.database_name
            )
        }
    }

    pub fn load_configurations(
        filename: &str,
        format: FileFormat,
    ) -> Result<DatabaseSettings, config::ConfigError> {
        let file: File<FileSourceFile, FileFormat> = config::File::new(filename, format);

        Config::builder()
            .add_source(file)
            .build()?
            .try_deserialize()
    }
}
