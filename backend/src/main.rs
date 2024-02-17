mod controller;
mod setting;
use actix_web::{web, App, HttpServer}; // 同じ階層にあるsettingモジュールをインポート

const DATABASE_URL: &str = "mysql://user:password@localhost:3306";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // DB説zpく
    let conn: sea_orm::prelude::DatabaseConnection =
        sea_orm::Database::connect(DATABASE_URL).await.unwrap();
    let state: setting::AppState = setting::AppState { conn };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(controller::health_check::hello)
    })
    .bind(("0.0.0.0", 8080))? // docker の場合、0.0.0.0 で listen する必要がある
    .run()
    .await
}
