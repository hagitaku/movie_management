use crate::setting::AppState;
use actix_web::{get, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Serialize)]
struct ResponseBody {
    message: String,
}

#[get("/health_check")]
pub async fn health_check(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn: &DatabaseConnection = &data.conn;
    match conn.ping().await {
        Ok(_) => {
            let res = ResponseBody {
                message: "Database connection is OK".to_string(),
            };
            Ok(HttpResponse::Ok().json(res))
        }
        Err(_) => {
            let res = ResponseBody {
                message: "Database connection is NG".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(res))
        }
    }
}
