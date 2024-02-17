use crate::setting::AppState;
use actix_web::{get, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;
use std::result::Result;

#[get("/")]
pub async fn hello(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn: &DatabaseConnection = &data.conn;
    match conn.ping().await {
        Ok(_) => Ok(HttpResponse::Ok().body("Database connection is OK")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Database connection failed")),
    }
}
