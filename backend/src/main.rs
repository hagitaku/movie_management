mod controller;
mod db_connector;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db_connector::db_init();

    HttpServer::new(|| {
        App::new()
            .service(controller::health_check::hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))? // docker の場合、0.0.0.0 で listen する必要がある
    .run()
    .await
}
