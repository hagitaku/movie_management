use crate::constants::failed_messages::PROCESS_FAILED_WITH_ACCOUNT;
use crate::constants::regex::{LOGIN_ID_REGEX, PASSWORD_REGEX};
use crate::constants::validation::{LOGIN_ID_MAX_LENGTH, PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH};
use crate::form::account_manage::AccountRegisterRequest;
use actix_web::web;
use regex::Regex;

fn valid_login_id(login_id: &String) -> bool {
    if login_id.len() > LOGIN_ID_MAX_LENGTH {
        return false;
    }
    let re = Regex::new(LOGIN_ID_REGEX).unwrap();
    return re.is_match(login_id);
}

fn valid_pass_word(password: &String) -> bool {
    if password.len() < PASSWORD_MIN_LENGTH || password.len() > PASSWORD_MAX_LENGTH {
        return false;
    }
    let re1 = Regex::new(PASSWORD_REGEX).unwrap();
    // パスワードにアルファベットが含まれているかチェック
    let has_alpha = password.chars().any(|c| c.is_alphabetic());
    // パスワードに数字が含まれているかチェック
    let has_digit = password.chars().any(|c| c.is_digit(10));

    if !has_alpha || !has_digit {
        return false;
    }
    return re1.is_match(password);
}

/**
 * アカウントの登録・ログインのバリデーション
 * 同一のバリデーションを使用する
 * 登録の型はAccountRegisterRequest、ログインの型はLoginRequest
 * どちらもログインIDとパスワードを持つ
 */
pub trait AccountCredentials {
    fn id(&self) -> &String;
    fn password(&self) -> &String;
}

impl AccountCredentials for AccountRegisterRequest {
    fn id(&self) -> &String {
        &self.id
    }
    fn password(&self) -> &String {
        &self.password
    }
}

use crate::form::account_manage::LoginRequest;
impl AccountCredentials for LoginRequest {
    fn id(&self) -> &String {
        &self.id
    }
    fn password(&self) -> &String {
        &self.password
    }
}

pub fn validation_account_manage<T: AccountCredentials>(request: &web::Json<T>) -> &str {
    let login_id = request.id();
    let password = request.password();

    if !valid_login_id(login_id) || !valid_pass_word(password) {
        return PROCESS_FAILED_WITH_ACCOUNT;
    }

    return "";
}
