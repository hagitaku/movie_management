use crate::form::health_check::HealthCheckResponse;
use crate::setting::AppState;
use actix_web::{get, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    get,
    path = "/health_check",
    responses(
        (status = 200, description = "ヘルスチェック用エンドポイント", body = HealthCheckResponse)
    )
)]
#[get("/health_check")]
pub async fn health_check(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn: &DatabaseConnection = &data.conn;
    match conn.ping().await {
        Ok(_) => {
            let res = HealthCheckResponse {
                message: "Database connection is OK".to_string(),
            };
            Ok(HttpResponse::Ok().json(res))
        }
        Err(_) => {
            let res = HealthCheckResponse {
                message: "Database connection is NG".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(res))
        }
    }
}
