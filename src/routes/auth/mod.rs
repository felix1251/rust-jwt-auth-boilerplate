pub mod handlers;

// use crate::middleware::auth_user;
use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

pub fn routes(db: DatabaseConnection) -> Router {
    Router::new()
        // .route_layer(from_fn_with_state(db.clone(), auth_user))
        .route("/sign_in", post(handlers::sign_in))
        .with_state(db)
}
