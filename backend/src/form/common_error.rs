use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct CommonErrorResponseBody {
    message: String,
}

// 500系共通のエラーレスポンス
pub fn internal_server_error() -> CommonErrorResponseBody {
    return CommonErrorResponseBody {
        message: "Internal Server Error".to_string(),
    };
}
