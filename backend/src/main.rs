mod controller;
mod setting;
use actix_web::{web, App, HttpServer}; // 同じ階層にあるsettingモジュールをインポート
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // .envを読み込んで環境変数にセットする
    dotenv().ok();
    let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_pass = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
    let database_url = format!(
        "{}{}{}{}{}",
        "mysql://", database_user, ":", database_pass, "@0.0.0.0:3306"
    );
    // デバッグよう
    println!("{}", database_url);

    // DB接続
    let conn: sea_orm::prelude::DatabaseConnection =
        sea_orm::Database::connect(database_url).await.unwrap();
    let state: setting::AppState = setting::AppState { conn };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(controller::health_check::health_check)
    })
    .bind(("0.0.0.0", 8080))? // docker の場合、0.0.0.0 で listen する必要がある
    .run()
    .await
}
