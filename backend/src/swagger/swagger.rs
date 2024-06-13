use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controller::health_check::health_check,
        crate::controller::movie_manage::movie_register
    ),
    components(schemas(
        crate::form::commom_error::CommonErrorResponseBody,
        crate::form::health_check::HealthCheckResponse,
        crate::form::movie_registration::MovieRegistrationRequest,
        crate::form::movie_registration::MovieRegistrationResponse
    ))
)]
pub struct ApiDoc;
