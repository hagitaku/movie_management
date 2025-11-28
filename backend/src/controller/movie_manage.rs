use crate::form::common_error::internal_server_error;
use crate::form::common_error::CommonErrorResponseBody;
use crate::form::movie_create::MovieCreateRequest;
use crate::form::movie_create::MovieCreateResponse;
use crate::form::movie_list::MovieListRequest;
use crate::form::movie_list::MovieListResponse;
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
                message: "Your request_json is ".to_owned() + &request_json,
            };
            Ok(HttpResponse::Ok().json(res))
        }
        Err(_) => {
            let res: CommonErrorResponseBody = internal_server_error("");
            Ok(HttpResponse::InternalServerError().json(res))
        }
    }
}

// 映画の一覧を取得するapi
#[utoipa::path(
    post,
    path = "/movie/search",
    request_body = MovieListRequest,
    responses(
        (status = 200, description = "取得成功時", body = MovieListResponse),
        (status = 500, description = "エラー発生時", body = CommonErrorResponseBody),
    )
)]
#[post("/movie/search")]
pub async fn movie_list(
    _data: web::Data<AppState>,
    request: web::Json<MovieListRequest>,
) -> Result<HttpResponse, Error> {
    // mockのレスポンスを返す
    let count: i32 = request.count.unwrap_or(10);
    let page: i32 = request.page.unwrap_or(1);
    // countとpageに応じてレスポンスを変える
    let mut movie_list: Vec<crate::form::movie_list::MovieListItem> = Vec::new();
    for i in 0..count {
        let movie_id = (page - 1) * count + i + 1;
        let item = crate::form::movie_list::MovieListItem {
            movie_id,
            title: format!("Movie Title {}", movie_id),
            description: format!("Description for movie {}", movie_id),
            user_name: format!("User {}", movie_id),
            created_date: "2024-01-01".to_string(),
            user_id: movie_id + 1000,
        };
        movie_list.push(item);
    }
    let res = crate::form::movie_list::MovieListResponse {
        movie_list,
        total_count: 100, // 仮の総件数
        page,
        count,
    };
    Ok(HttpResponse::Ok().json(res))
}
