use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use persistence_interface::repository::server_configuration_repository::ServerConfigurationRepository;
use server::{init_database_operations, Settings};
use std::net::TcpListener;


#[tokio::main]
async fn main() {
    let settings = Settings::read("./server/resources/app.yaml");

    log::info!("app settings = {settings:?}");

    let db_operations =
        init_database_operations(&settings.database.connection_string)
            .await;

    let configuration = db_operations
        .get_server_configuration()
        .await
        .expect("failed to get server configuration");

    let port = configuration.port;

    let tcp_listener =
        TcpListener::bind(format!("127.0.0.1:{port}"))
            .expect("failed to bind TcpListener");

    let db_ops_data =
        web::Data::new(db_operations);

    HttpServer::new(move || {
        App::new()
            .app_data(db_ops_data.clone())
    }).listen(tcp_listener)
        .expect("could not lister to TcpListener")
        .run()
        .await
        .expect("server failed");
}
