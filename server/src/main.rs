use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use server::{init_database_operations, Settings};
use std::net::TcpListener;

#[tokio::main]
async fn main() {
    let settings = Settings::read("./resources/app.yaml");

    log::info!("app settings = {settings:?}");

    let db_operations = init_database_operations(&settings.database.connection_string).await;

    let updated_entries = db_operations
        .migrate("./sql/schema.sql")
        .await
        .expect("failed database migration");

    log::info!("migration updated entries = {updated_entries}");

    let configuration = db_operations
        .get_server_configuration()
        .await
        .expect("failed to get server configuration");

    let port = configuration.port;
    let protocol = if configuration.use_https {
        "https"
    } else {
        "http"
    };

    let tcp_listener = TcpListener::bind(format!("{protocol}://localhost:{port}"))
        .expect("failed to bind TcpListener");

    HttpServer::new(move || App::new().app_data(web::Data::new(db_operations)))
        .listen(tcp_listener)
        .expect("could not lister to TcpListener")
        .run()
        .await
        .expect("server failed");
}
