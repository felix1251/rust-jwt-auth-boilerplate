mod examples;
mod home;

use axum::{
    http::Method,
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::middleware::{authenticate_user, fallback};

pub fn create_routes() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any);

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
                .route_layer(from_fn(authenticate_user)),
        )
        // Trace layer for logging
        .layer(TraceLayer::new_for_http())
        // Cors layer
        .layer(cors)
        // 404 not found fallback
        .fallback(fallback)
}
