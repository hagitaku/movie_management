use serde::Serialize;

#[derive(Serialize)]
pub struct MovieRegistrationRequest {
    pub title: String,
    pub description: String,
    pub memo: String,
    pub session_token: String,
}

#[derive(Serialize)]
pub struct MovieRegistrationResponse {
    pub message: String,
}
