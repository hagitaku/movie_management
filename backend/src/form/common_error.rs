use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct CommonErrorResponseBody {
    message: String,
}

// 500系共通のエラーレスポンス
// message : エラーメッセージ(非必須でデフォルトで空文字列)
pub fn internal_server_error(message: &str) -> CommonErrorResponseBody {
    return CommonErrorResponseBody {
        message: "Internal Server Error : ".to_string() + &message,
    };
}
