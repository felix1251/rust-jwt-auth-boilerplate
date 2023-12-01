pub mod schemas;
use crate::routes::home;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "JWT Auth", description = "JWT Auth boilerplate"),
    paths(home::home),
    components(schemas(schemas::home::HomeSchema))
)]
struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/api/documentation").url("/api/documentation.json", ApiDoc::openapi())
}
