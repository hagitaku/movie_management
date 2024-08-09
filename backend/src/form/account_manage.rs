use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
// ---------------------------------------------------------
// アカウント登録IF
#[derive(Serialize, Deserialize, ToSchema)]
pub struct AccountRegisterRequest {
    pub id: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct AccountRegisterResponse {
    pub message: String,
}

// ---------------------------------------------------------
// ログインIF
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub id: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub message: String,
}
