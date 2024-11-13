use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use server::{init_database_operations, Settings};

#[tokio::main]
async fn main() {
    let config = Settings::read("./resources/app.yaml");

    let db_operations = init_database_operations(&config.database.connection_string).await;

    HttpServer::new(move || App::new().app_data(web::Data::new(db_operations)));
}
