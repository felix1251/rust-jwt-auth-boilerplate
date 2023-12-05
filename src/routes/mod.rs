mod examples;
pub mod home;
pub mod users;

use crate::middleware::{auth_user, cors, fallback};
use crate::swagger::swagger_ui;
use axum::Extension;
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

pub async fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        // Home path /
        .route("/", get(home::home))
        // users routes
        .nest(
            "/v1",
            Router::new().nest(
                "/users",
                Router::new()
                    .route("/me", get(users::me))
                    .route_layer(from_fn(auth_user))
                    .route("/sign_in", post(users::sign_in)),
            ),
        )
        // Database Layer
        .layer(Extension(db))
        // Trace layer for logging
        .layer(TraceLayer::new_for_http())
        // Cors layer
        .layer(cors())
        // Swagger UI layer
        .merge(swagger_ui())
        // 404 not found fallback
        .fallback(fallback)
}
