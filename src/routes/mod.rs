mod examples;
mod home;

use crate::middleware::{auth_user, cors, fallback};
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Category {
    pub id: usize,
    pub name: String,
    pub url: String,
    pub icon: String,
}

#[derive(OpenApi)]
#[openapi(
    info(title = "JWT Auth", description = "JWT Auth boilerplate"),
    paths(home::home),
    components(schemas(home::Home))
)]
struct ApiDoc;

pub fn create_routes() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    Router::new()
        // Home path /
        .route("/", get(home::home))
        // example routes
        .nest(
            "/examples",
            Router::new()
                .route("/test_json", post(examples::test_json))
                .route("/path_vars/:id", get(examples::path_vars))
                .route("/query_params", get(examples::query_params))
                .route("/headers", get(examples::headers))
                // Auth Middleware
                // Isolate route with nest to allow auth middleware only in a scope (Ex. /v1/... or /v2/..)
                .route_layer(from_fn(auth_user)),
        )
        // Trace layer for logging
        .layer(TraceLayer::new_for_http())
        // Cors layer
        .layer(cors())
        .merge(
            SwaggerUi::new("/api/documentation").url("/api/documentation.json", ApiDoc::openapi()),
        )
        // 404 not found fallback
        .fallback(fallback)
}
