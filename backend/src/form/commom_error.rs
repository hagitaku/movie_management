use serde::Serialize;

#[derive(Serialize)]
pub struct CommonErrorResponseBody {
    message: String,
}

// 500系共通のエラーレスポンス
pub fn internal_server_error() -> CommonErrorResponseBody {
    return CommonErrorResponseBody {
        message: "Internal Server Error".to_string(),
    };
}
