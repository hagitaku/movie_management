use crate::form::common_error::internal_server_error;
use crate::form::common_error::CommonErrorResponseBody;
use crate::form::movie_create::MovieCreateRequest;
use crate::form::movie_create::MovieCreateResponse;
use crate::setting::AppState;
use actix_web::{post, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    post,
    path = "/movie/register",
    request_body = MovieCreateRequest,
    responses(
        (status = 200, description = "登録成功時", body = MovieCreateResponse),
        (status = 500, description = "エラー発生時", body = CommonErrorResponseBody),
    )
)]
// 映画を登録するapi
#[post("/movie/register")]
pub async fn movie_register(
    data: web::Data<AppState>,
    request: web::Json<MovieCreateRequest>,
) -> Result<HttpResponse, Error> {
    let conn: &DatabaseConnection = &data.conn;
    match conn.ping().await {
        Ok(_) => {
            // requestをjson文字列に変換
            let request_json = serde_json::to_string(&request.0).unwrap();

            let res: MovieCreateResponse = MovieCreateResponse {
                message: "Your request_json is ".to_string() + &request_json,
            };
            Ok(HttpResponse::Ok().json(res))
        }
        Err(_) => {
            let res: CommonErrorResponseBody = internal_server_error("");
            Ok(HttpResponse::InternalServerError().json(res))
        }
    }
}
