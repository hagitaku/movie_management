use crate::controller::orm::account_manage::account_id_from_session;
use crate::controller::orm::movie_manage::insert_movie;
use crate::controller::validation::movie_management::validation_movie_management;
use crate::form::common_error::CommonErrorResponseBody;
use crate::form::movie_create::{MovieCreateRequest, MovieCreateResponse};
use crate::form::movie_list::{MovieListItem, MovieListRequest, MovieListResponse};
use crate::setting::AppState;
use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

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
        (status = 500, description = "エラー発生", body = CommonErrorResponseBody),
    )
)]
#[post("/movie/search")]
pub async fn movie_search(
    _data: web::Data<AppState>,
    request: web::Json<MovieListRequest>,
) -> Result<HttpResponse, Error> {
    let count = request.count;
    let page = request.page;
    let mut movie_list: Vec<MovieListItem> = Vec::new();

    for i in 0..count {
        let movie_id = (page - 1) * count + i + 1;
        movie_list.push(MovieListItem {
            movie_id,
            title: format!("Movie Title {movie_id}"),
            description: format!("Description for movie {movie_id}"),
            user_name: format!("User {movie_id}"),
            created_date: "2024-01-01".to_owned(),
            user_id: movie_id + 1000,
        });
    }

    Ok(HttpResponse::Ok().json(MovieListResponse {
        movie_list,
        total_count: 100,
        page,
        count,
    }))
}
