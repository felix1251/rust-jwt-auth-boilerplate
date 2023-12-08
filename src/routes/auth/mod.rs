pub mod handlers;

// use crate::middleware::auth_user;
use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

pub fn routes(db: DatabaseConnection) -> Router {
    Router::new()
        // .route_layer(from_fn_with_state(db.clone(), auth_user))
        .route("/sign-in", get(handlers::sign_in))
        .with_state(db)
}
