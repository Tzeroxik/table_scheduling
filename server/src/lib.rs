use config::FileFormat;
use persistence_impl_postgres::PostgresDatabaseOperations;
use persistence_interface::DatabaseOperations;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub connection_string: String,
}

impl Settings {
    pub fn read(file_path: &str) -> Settings {
        let file = config::File::new(file_path, FileFormat::Yaml);

        config::Config::builder()
            .add_source(file)
            .build()
            .expect("failed to build config")
            .try_deserialize()
            .expect("failed to deserialize config")
    }
}

pub async fn init_database_operations(
    connection_string: &str,
) -> impl DatabaseOperations {
    PostgresDatabaseOperations::from_con_str(connection_string)
        .await
        .expect("failed to initialize database")
}
