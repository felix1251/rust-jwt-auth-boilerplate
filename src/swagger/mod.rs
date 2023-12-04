use crate::routes::home::{self, HomeSchema};
use crate::routes::users::{self, UserMeSchema};
use serde::Serialize;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "JWT Auth", description = "JWT Auth boilerplate"),
    paths(home::home, users::me),
    components(schemas(HomeSchema, UserMeSchema, UnauthorizedSchema)),
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
    #[schema(example = 400)]
    pub status: u16,
    #[schema(example = "UNAUTHORIZED")]
    pub message: String,
}

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/api/docs").url("/api/docs.json", ApiDoc::openapi())
}
