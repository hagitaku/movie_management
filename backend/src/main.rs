mod constants;
mod controller;
mod form;
mod model;
mod setting;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod swagger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // .envを読み込んで環境変数にセットする
    dotenv().ok();
    let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_pass = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let database_url = format!(
        "{}{}{}{}{}{}",
        "mysql://", database_user, ":", database_pass, "@mysql:3306/", database_name
    );

    // DB接続
    let conn: sea_orm::prelude::DatabaseConnection =
        sea_orm::Database::connect(database_url).await.unwrap();
    let state: setting::AppState = setting::AppState { conn };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "OPTIONS", "DELETE", "PUT"])
            .allowed_header(http::header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .service(controller::health_check::health_check)
            .service(controller::movie_manage::movie_register)
            .service(controller::account_manage::account_registration)
            .service(controller::account_manage::auth_login)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/opanapi.json", swagger::swagger::ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", 8080))? // docker の場合、0.0.0.0 で listen する必要がある
    .run()
    .await
}
