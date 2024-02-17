mod controller;
mod db_connector;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db_connector::db_init();
    HttpServer::new(|| App::new().service(controller::health_check::hello))
        .bind(("0.0.0.0", 8080))? // docker の場合、0.0.0.0 で listen する必要がある
        .run()
        .await
}
