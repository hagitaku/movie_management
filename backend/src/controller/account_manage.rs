use crate::constants::failed_messages::PROCESS_FAILED_WITH_ACCOUNT;
use crate::controller::orm::account_manage::insert_account;
use crate::controller::validation::account_manage::validation_account_manage;
use crate::form::account_manage::{AccountRegisterRequest, AccountRegisterResponse};
use crate::form::account_manage::{LoginRequest, LoginResponse};
use crate::form::common_error::CommonErrorResponseBody;
use crate::setting::AppState;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, Error, HttpResponse};
use rand::Rng;

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
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(LoginResponse {
                message: PROCESS_FAILED_WITH_ACCOUNT.to_string(),
            }));
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
/**
 * ログインapi
 * ログインIDとパスワードを受け取り、認証を行う
 * 認証に成功した場合は、セッションIDをCookieにセットして返す
 * TODO: バリデーション失敗時、とログイン失敗時のレスポンスタイムを同一にするためにコントロールする
 * TODO: 生成したtokenをDBに保存する
 */
#[post("/auth/login")]
pub async fn auth_login(
    data: web::Data<AppState>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    let validation_result: &str = validation_account_manage(&request);

    if validation_result != "" {
        return core::result::Result::Ok(HttpResponse::BadRequest().json(LoginResponse {
            message: validation_result.to_string(),
        }));
    }

    let conn: &DatabaseConnection = &data.conn;

    match crate::controller::orm::account_manage::login_account(conn, &request).await {
        Ok(_) => {
            return Ok(create_login_response());
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(LoginResponse {
                message: PROCESS_FAILED_WITH_ACCOUNT.to_string(),
            }));
        }
    }
}

fn create_login_response() -> HttpResponse {
    // token用のランダムな32文字の文字列を生成
    let pass_word = generate_random_string(32);

    let cookie = Cookie::build("session", pass_word)
        .path("/")
        .http_only(true)
        .secure(false) // 本番環境ではtrueにすること
        .same_site(SameSite::Lax)
        .finish();

    HttpResponse::Ok().cookie(cookie).json(LoginResponse {
        message: "success".to_string(),
    })
}

/**
 * ランダムな値を生成する
 * 引数1: 文字列長
 * 引数2: 文字列の文字セット
 *
 * TODO: 他にも使用する予定が出てきたらutil.rsに移動する
 */
fn generate_random_string(length: usize) -> String {
    let pass_word = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>();

    return pass_word;
}
