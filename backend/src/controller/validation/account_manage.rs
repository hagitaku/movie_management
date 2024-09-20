use crate::constants::regex::{LOGIN_ID_REGEX, PASSWORD_1_REGEX};
use crate::form::account_manage::AccountRegisterRequest;
use actix_web::web;
use regex::Regex;

fn valid_login_id(login_id: &String) -> bool {
    if login_id.len() > 16 {
        return false;
    }
    let re = Regex::new(LOGIN_ID_REGEX).unwrap();
    return re.is_match(login_id);
}

fn valid_pass_word(password: &String) -> bool {
    if password.len() < 8 || password.len() > 32 {
        return false;
    }
    let re1 = Regex::new(PASSWORD_1_REGEX).unwrap();
    // パスワードにアルファベットが含まれているかチェック
    let has_alpha = password.chars().any(|c| c.is_alphabetic());
    // パスワードに数字が含まれているかチェック
    let has_digit = password.chars().any(|c| c.is_digit(10));

    if !has_alpha || !has_digit {
        return false;
    }
    return re1.is_match(password);
}

pub fn validation_account_manage(request: &web::Json<AccountRegisterRequest>) -> &str {
    let login_id = &request.id;
    let password = &request.password;

    if !valid_login_id(login_id) || !valid_pass_word(password) {
        return "ログインIDもしくはパスワードが不正です";
    }

    return "";
}
