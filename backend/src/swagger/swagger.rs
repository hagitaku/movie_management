use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controller::health_check::health_check,
        crate::controller::movie_manage::movie_register,
        crate::controller::account_manage::account_registration,
        crate::controller::account_manage::auth_login
    ),
    components(schemas(
        crate::form::common_error::CommonErrorResponseBody,
        crate::form::health_check::HealthCheckResponse,
        crate::form::movie_create::MovieCreateRequest,
        crate::form::movie_create::MovieCreateResponse,
        crate::form::account_manage::AccountRegisterRequest,
        crate::form::account_manage::AccountRegisterResponse,
        crate::form::account_manage::LoginRequest,
        crate::form::account_manage::LoginResponse
    ))
)]
pub struct ApiDoc;
