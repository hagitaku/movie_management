use crate::controller::validation::movie_management::validation_movie_management;
use crate::form::common_error::internal_server_error;
use crate::form::common_error::CommonErrorResponseBody;
use crate::form::movie_create::MovieCreateRequest;
use crate::form::movie_create::MovieCreateResponse;
use crate::form::movie_list::MovieListRequest;
use crate::form::movie_list::MovieListResponse;
use crate::setting::AppState;
use actix_web::{post, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

// #[utoipa::path(
//     post,
//     path = "/movie/register",
//     request_body = MovieCreateRequest,
//     responses(
//         (status = 200, description = "登録成功時", body = MovieCreateResponse),
//         (status = 500, description = "エラー発生時", body = CommonErrorResponseBody),
//     )
// )]
// // 映画を登録するapi
// #[post("/movie/register")]
// pub async fn movie_register(
//     data: web::Data<AppState>,
//     request: web::Json<MovieCreateRequest>,
// ) -> Result<HttpResponse, Error> {
//     data
//     let validation_result = validation_movie_management(&request);
//     if validation_result != "" {
//         let res: MovieCreateResponse = MovieCreateResponse {
//             message: validation_result,
//         };
//         return Ok(HttpResponse::BadRequest().json(res));
//     }

//     let conn: &DatabaseConnection = &data.conn;
//     // 映画登録処理
//     match internal_server_error() {
//         Ok(_) => {
//             return Ok(HttpResponse::Ok().json(MovieCreateResponse {
//                 message: "success".to_owned(),
//             }));
//         }
//         Err(_) => {
//             return Ok(
//                 HttpResponse::InternalServerError().json(CommonErrorResponseBody {
//                     message: "映画の登録に失敗しました".to_owned(),
//                 }),
//             );
//         }
//     }
// }

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
pub async fn movie_search(
    _data: web::Data<AppState>,
    request: web::Json<MovieListRequest>,
) -> Result<HttpResponse, Error> {
    // mockのレスポンスを返す
    let count: i32 = request.count;
    let page: i32 = request.page;

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
