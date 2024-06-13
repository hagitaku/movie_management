use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]

pub struct HealthCheckResponse {
    pub message: String,
}
