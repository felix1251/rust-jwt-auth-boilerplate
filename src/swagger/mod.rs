use crate::routes::auth;
use crate::routes::home;
// use crate::routes::users;
use crate::utils::jwt;
use serde::Serialize;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "JWT Auth", description = "JWT Auth boilerplate"),
    paths(
        home::home,
        auth::handlers::me,
        auth::handlers::sign_in,
        auth::handlers::sign_up,
        auth::handlers::refresh_token
    ),
    components(
        schemas(
            UnauthorizedSchema,
            InternalErrorSchema,
            ValidationErrorSchema,
            UserExistOrInvalidSchema,
            InvalidCredentialSchema,
            jwt::AuthTokens,
            home::HomeSchema,
            auth::handlers::SignInParams,
            auth::handlers::SignUpParams,
            auth::handlers::CurrentUser,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints.")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .description("Enter JWT token".into())
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(ToSchema, Serialize)]
struct UnauthorizedSchema {
    #[schema(example = 401)]
    pub status: u16,
    #[schema(example = "UNAUTHORIZED")]
    pub error: &'static str,
}

#[derive(ToSchema, Serialize)]
struct InternalErrorSchema {
    #[schema(example = 500)]
    pub status: u16,
    #[schema(example = "INTERNAL_SERVER_ERROR")]
    pub error: &'static str,
}

#[derive(ToSchema, Serialize)]
struct ValidationErrorSchema {
    #[schema(example = 422)]
    pub status: u16,
    #[schema(example = json!({"email": [{"code": "email", "message": "Invalid Email", "params": { "value": "sample" }}]}))]
    pub error: &'static str,
}

#[derive(ToSchema, Serialize)]
struct UserExistOrInvalidSchema {
    #[schema(example = 409)]
    pub status: u16,
    #[schema(example = "USER_EXIST_OR_INVALID")]
    pub error: &'static str,
}

#[derive(ToSchema, Serialize)]
struct InvalidCredentialSchema {
    #[schema(example = 404)]
    pub status: u16,
    #[schema(example = "INVALID_CREDENTIALS")]
    pub error: &'static str,
}

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/api/docs").url("/api/docs.json", ApiDoc::openapi())
}
