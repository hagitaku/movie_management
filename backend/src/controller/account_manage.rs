use crate::controller::orm::account_manage::insert_account;
use crate::controller::validation::account_manage::validation_account_manage;
use crate::form::account_manage::{AccountRegisterRequest, AccountRegisterResponse};
use crate::form::account_manage::{LoginRequest, LoginResponse};
use crate::form::common_error::{internal_server_error, CommonErrorResponseBody};
use crate::setting::AppState;
use actix_web::{post, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    post,
    path = "/account/register",
    request_body = AccountRegisterRequest,
    responses(
        (status = 200, description = "登録成功時", body = AccountRegisterResponse),
        (status = 400, description = "登録失敗時", body = AccountRegisterResponse),
        (status = 500, description = "エラー発生時", body = CommonErrorResponseBody),
    )
)]
// アカウントを登録するapi
#[post("/account/register")]
pub async fn account_registration(
    data: web::Data<AppState>,
    request: web::Json<AccountRegisterRequest>,
) -> Result<HttpResponse, Error> {
    let validation_result: &str = validation_account_manage(&request);
    if validation_result != "" {
        return core::result::Result::Ok(HttpResponse::BadRequest().json(
            AccountRegisterResponse {
                message: validation_result.to_string(),
            },
        ));
    }

    let conn: &DatabaseConnection = &data.conn;

    match insert_account(conn, &request).await {
        Ok(_) => {
            return Ok(HttpResponse::Ok().json(AccountRegisterResponse {
                message: "success".to_string(),
            }));
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(internal_server_error(e.to_string().as_str())))
        }
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "ログイン成功時", body = LoginResponse),
        (status = 400, description = "ログイン失敗時", body = LoginResponse),
        (status = 500, description = "エラー発生時", body = CommonErrorResponseBody),
    )
)]
// ログインapi
#[post("/auth/login")]
pub async fn auth_login(
    data: web::Data<AppState>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    let conn: &DatabaseConnection = &data.conn;

    match conn.ping().await {
        Ok(_) => {
            let pass_word = request.0.password;

            Ok(create_login_response(pass_word))
        }
        Err(_) => {
            let res: CommonErrorResponseBody = internal_server_error("");
            Ok(HttpResponse::InternalServerError().json(res))
        }
    }
}

fn create_login_response(pass_word: String) -> HttpResponse {
    if pass_word == "ng" {
        return HttpResponse::BadRequest().json(LoginResponse {
            message: "failed".to_string(),
        });
    } else {
        return HttpResponse::Ok().json(LoginResponse {
            message: "success".to_string(),
        });
    }
}
