use config::FileFormat;
use persistence_impl_postgres::repository::server_configuration::PostgresServerConfigurationRepository;
use persistence_interface::dto::server_configuration_repository::ServerConfigurationRepository;
use sqlx::PgPool;

async fn get_connection_pool() -> PgPool {
    let settings = configuration::load_configurations(
        "./tests/database.yaml",
        FileFormat::Yaml,
    ).expect("Failed to load configurations.yml");

    PgPool::connect(&settings.connection_string())
        .await
        .expect("Failed to connect to Postgres")
}

#[tokio::test]
async fn test_get_server_configuration() {
    let pool = get_connection_pool().await;

    let repository =
        PostgresServerConfigurationRepository::from_connection_pool(pool);

    let configuration =
        repository
            .get_server_configuration()
            .await
            .expect("Failed to get server configuration");

    println!("{configuration:?}");
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
        let file: File<FileSourceFile, FileFormat> =
            config::File::new(filename, format);

        Config::builder()
            .add_source(file)
            .build()?
            .try_deserialize()
    }
}