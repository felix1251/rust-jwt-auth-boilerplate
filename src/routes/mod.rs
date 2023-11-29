mod examples;
mod home;

use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

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
                .route_layer(middleware::from_fn(authenticate_user)),
        )
        // Trace layer for logging
        .layer(TraceLayer::new_for_http())
        // Cors layer
        .layer(cors)
        // 404 not found fallback
        .fallback(fallback)
}

#[derive(Serialize)]
pub struct ErrorRes {
    pub status: u16,
    pub message: &'static str,
}

async fn fallback() -> (StatusCode, Json<ErrorRes>) {
    let status = StatusCode::NOT_FOUND;

    let err_res = ErrorRes {
        status: status.as_u16(),
        message: "ROUTE_NOT_FOUND",
    };

    (status, Json(err_res))
}

async fn authenticate_user(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrorRes>)> {
    let headers = request.headers();

    let _auth_header = headers
        .get("Authorization")
        .ok_or_else(|| {
            let status = StatusCode::UNAUTHORIZED;
            let err_res = ErrorRes {
                status: status.as_u16(),
                message: "UNAUTHORIZED",
            };

            (status, Json(err_res))
        })?
        .to_str();

    // some logic here to check if the auth header is a valid JWT token

    Ok(next.run(request).await)
}
