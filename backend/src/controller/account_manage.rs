use crate::constants::failed_messages::PROCESS_FAILED_WITH_ACCOUNT;
use crate::controller::orm::account_manage::{insert_account, replace_session};
use crate::controller::validation::account_manage::validation_account_manage;
use crate::form::account_manage::{AccountRegisterRequest, AccountRegisterResponse};
use crate::form::account_manage::{LoginRequest, LoginResponse};
use crate::form::common_error::CommonErrorResponseBody;
use crate::setting::AppState;
use actix_session::Session;
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
    let validation_result: String = validation_account_manage(&request);
    if validation_result != "" {
        return core::result::Result::Ok(HttpResponse::BadRequest().json(
            AccountRegisterResponse {
                message: validation_result,
            },
        ));
    }

    let conn: &DatabaseConnection = &data.conn;

    match insert_account(conn, &request).await {
        Ok(_) => {
            return Ok(HttpResponse::Ok().json(AccountRegisterResponse {
                message: "success".to_owned(),
            }));
        }
        Err(_) => {
            return Ok(
                HttpResponse::InternalServerError().json(AccountRegisterResponse {
                    message: PROCESS_FAILED_WITH_ACCOUNT.to_owned(),
                }),
            );
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
    session: Session,
) -> Result<HttpResponse, Error> {
    let validation_result: String = validation_account_manage(&request);

    if validation_result != "" {
        return core::result::Result::Ok(HttpResponse::BadRequest().json(LoginResponse {
            message: validation_result,
        }));
    }

    let conn: &DatabaseConnection = &data.conn;

    match crate::controller::orm::account_manage::login_account(conn, &request).await {
        Ok(account) => {
            let session_token = generate_random_string(32);
            if replace_session(conn, account.id, &session_token)
                .await
                .is_err()
            {
                return Ok(
                    HttpResponse::InternalServerError().json(CommonErrorResponseBody {
                        message: "セッションの作成に失敗しました".to_owned(),
                    }),
                );
            }
            return create_login_response(session, session_token);
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(LoginResponse {
                message: PROCESS_FAILED_WITH_ACCOUNT.to_owned(),
            }));
        }
    }
}

fn create_login_response(session: Session, session_token: String) -> Result<HttpResponse, Error> {
    // token用のランダムな32文字の文字列を生成
    session.insert("session_token", session_token)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        message: "success".to_owned(),
    }))
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
