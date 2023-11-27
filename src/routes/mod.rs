mod examples;
mod home;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

pub fn create_routes() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    Router::new()
        .route("/", get(home::home))
        .nest(
            "/examples",
            Router::new()
                .route("/test_json", post(examples::test_json))
                .route("/path_vars/:id", get(examples::path_vars))
                .route("/query_params", get(examples::query_params))
                .route("/headers", get(examples::headers)),
        )
        .layer(TraceLayer::new_for_http())
        .fallback(fallback)
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "ROUTE_NOT_FOUND")
}
