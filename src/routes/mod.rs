mod examples;
pub mod home;

use crate::middleware::{auth_user, cors, fallback};
use crate::swagger::swagger_ui;
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

#[derive(Serialize, Deserialize)]
pub struct ErrRes {
    pub status: u16,
    pub message: &'static str,
}

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
        .merge(swagger_ui())
        // 404 not found fallback
        .fallback(fallback)
}
