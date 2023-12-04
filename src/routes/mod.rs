mod examples;
pub mod home;
pub mod users;

use crate::middleware::{auth_user, cors, fallback};
use crate::swagger::swagger_ui;
use axum::Extension;
use axum::{middleware::from_fn, routing::get, Router};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

#[derive(Serialize, Deserialize)]
pub struct ErrRes {
    pub status: u16,
    pub message: &'static str,
}

pub async fn create_routes(db: DatabaseConnection) -> Router {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    Router::new()
        // Home path /
        .route("/", get(home::home))
        // users routes
        .nest(
            "/v1",
            Router::new()
                .nest("/users", Router::new().route("/me", get(users::me)))
                .route_layer(from_fn(auth_user)),
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
