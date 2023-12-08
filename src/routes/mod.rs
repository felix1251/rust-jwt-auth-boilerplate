pub mod home;
pub mod users;

use crate::middleware::{cors, fallback};
use crate::swagger::swagger_ui;
use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

pub async fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        // Home path /
        .route("/", get(home::home))
        .nest(
            "/v1",
            Router::new()
                // users routes
                .nest("/users", users::routes(db.clone())),
        )
        // Database Layer
        // Trace layer for logging
        .layer(TraceLayer::new_for_http())
        // Cors layer
        .layer(cors())
        // Swagger UI layer
        .merge(swagger_ui())
        // 404 not found fallback
        .fallback(fallback)
}
