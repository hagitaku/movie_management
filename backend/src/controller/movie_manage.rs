use crate::form::commom_error::internal_server_error;
use crate::form::commom_error::CommonErrorResponseBody;
use crate::form::movie_registration::MovieRegistrationRequest;
use crate::form::movie_registration::MovieRegistrationResponse;
use crate::setting::AppState;
use actix_web::post;
use actix_web::{web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    post,
    path = "/movie/register",
    request_body = MovieRegistrationRequest,
    responses(
        (status = 200, description = "ヘルスチェック用エンドポイント", body = MovieRegistrationResponse),
        (status = 500, description = "エラー発生時", body = CommonErrorResponseBody),
    )
)]
// 映画を登録するapi
#[post("/movie/register")]
pub async fn movie_register(
    data: web::Data<AppState>,
    request: web::Json<MovieRegistrationRequest>,
) -> Result<HttpResponse, Error> {
    let conn: &DatabaseConnection = &data.conn;
    match conn.ping().await {
        Ok(_) => {
            // requestをjson文字列に変換
            let request_json = serde_json::to_string(&request.0).unwrap();

            let res: MovieRegistrationResponse = MovieRegistrationResponse {
                message: "Your request_json is ".to_string() + &request_json,
            };
            Ok(HttpResponse::Ok().json(res))
        }
        Err(_) => {
            let res: CommonErrorResponseBody = internal_server_error();
            Ok(HttpResponse::InternalServerError().json(res))
        }
    }
}
