use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MovieCreateRequest {
    pub title: String,
    pub description: String,
    pub memo: String,
    pub session_token: String,
    // 以下、TODO
    // pub genre: Genre, NOTE: Genre構造体はIDと名前を持ち、IDが外部キーとして使われる。IDが存在しない場合は新規登録される。
    // pub url: String, NOTE: URLは空文字列を許容する。
    // pub image: String, NOTE: 画像は空文字列を許容する。
}

#[derive(Serialize, ToSchema)]
pub struct MovieCreateResponse {
    pub message: String,
}
