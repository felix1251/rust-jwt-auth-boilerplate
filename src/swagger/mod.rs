use crate::routes::home::{self, HomeSchema};
use crate::routes::users::{self, UserMeSchema};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "JWT Auth", description = "JWT Auth boilerplate"),
    paths(home::home, users::me),
    components(schemas(HomeSchema, UserMeSchema, UnauthorizedSchema))
)]
struct ApiDoc;

#[derive(ToSchema, Serialize)]
#[schema(example = json!({"status": 401, "message": "UNAUTHORIZED"}))]
struct UnauthorizedSchema {
    pub status: u16,
    pub message: String,
}

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/api/documentation").url("/api/documentation.json", ApiDoc::openapi())
}
