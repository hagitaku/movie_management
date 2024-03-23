use crate::setting::AppState;
use actix_web::post;
use actix_web::{web, Error, HttpResponse};
use sea_orm::DatabaseConnection;
// ../form/movie_registration.rs から MovieRegistrationRequest と MovieRegistrationResponse をインポート
use crate::form::commom_error::internal_server_error;
use crate::form::commom_error::CommonErrorResponseBody;
use crate::form::movie_registration::{MovieRegistrationRequest, MovieRegistrationResponse};

// 映画を登録するapi
#[post("/movie/register")]
pub async fn movie_register(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn: &DatabaseConnection = &data.conn;
    match conn.ping().await {
        Ok(_) => {
            let res: MovieRegistrationResponse = MovieRegistrationResponse {
                message: "Database connection is OK".to_string(),
            };
            Ok(HttpResponse::Ok().json(res))
        }
        Err(_) => {
            let res: CommonErrorResponseBody = internal_server_error();
            Ok(HttpResponse::InternalServerError().json(res))
        }
    }
}
