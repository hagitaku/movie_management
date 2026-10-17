use crate::controller::orm::account_manage::account_id_from_session;
use crate::controller::orm::movie_manage::{insert_movie, search_movies};
use crate::controller::validation::movie_management::validation_movie_management;
use crate::form::common_error::CommonErrorResponseBody;
use crate::form::movie_create::{MovieCreateRequest, MovieCreateResponse};
use crate::form::movie_list::{MovieListRequest, MovieListResponse};
use crate::setting::AppState;
use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

#[cfg(test)]
#[path = "movie_manage_tests.rs"]
mod tests;

#[utoipa::path(
    post,
    path = "/movie/register",
    request_body = MovieCreateRequest,
    responses(
        (status = 200, description = "映画登録成功", body = MovieCreateResponse),
        (status = 400, description = "入力値不正", body = MovieCreateResponse),
        (status = 401, description = "未ログイン", body = CommonErrorResponseBody),
        (status = 500, description = "エラー発生", body = CommonErrorResponseBody),
    )
)]
#[post("/movie/register")]
pub async fn movie_register(
    data: web::Data<AppState>,
    request: web::Json<MovieCreateRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_token = match session.get::<String>("session_token")? {
        Some(token) if !token.is_empty() => token,
        _ => {
            return Ok(HttpResponse::Unauthorized().json(CommonErrorResponseBody {
                message: "ログインが必要です".to_owned(),
            }))
        }
    };

    let conn: &DatabaseConnection = &data.conn;
    let account_id = match account_id_from_session(conn, &session_token).await {
        Ok(Some(account_id)) => account_id,
        Ok(None) => {
            return Ok(HttpResponse::Unauthorized().json(CommonErrorResponseBody {
                message: "セッションが無効です".to_owned(),
            }))
        }
        Err(_) => {
            return Ok(
                HttpResponse::InternalServerError().json(CommonErrorResponseBody {
                    message: "セッションの確認に失敗しました".to_owned(),
                }),
            )
        }
    };

    let validation_result = validation_movie_management(&request);
    if !validation_result.is_empty() {
        return Ok(HttpResponse::BadRequest().json(MovieCreateResponse {
            message: validation_result,
        }));
    }

    match insert_movie(conn, &request, account_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(MovieCreateResponse {
            message: "success".to_owned(),
        })),
        Err(_) => Ok(
            HttpResponse::InternalServerError().json(CommonErrorResponseBody {
                message: "映画の登録に失敗しました".to_owned(),
            }),
        ),
    }
}

#[utoipa::path(
    post,
    path = "/movie/search",
    request_body = MovieListRequest,
    responses(
        (status = 200, description = "映画一覧取得", body = MovieListResponse),
        (status = 400, description = "検索条件不正", body = CommonErrorResponseBody),
        (status = 500, description = "エラー発生", body = CommonErrorResponseBody),
    )
)]
#[post("/movie/search")]
pub async fn movie_search(
    data: web::Data<AppState>,
    request: web::Json<MovieListRequest>,
) -> Result<HttpResponse, Error> {
    if let Err(message) = request.validate() {
        return Ok(HttpResponse::BadRequest().json(CommonErrorResponseBody {
            message: message.to_owned(),
        }));
    }

    match search_movies(&data.conn, &request).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(
            HttpResponse::InternalServerError().json(CommonErrorResponseBody {
                message: "映画一覧の取得に失敗しました".to_owned(),
            }),
        ),
    }
}
