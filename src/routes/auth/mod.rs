pub mod handlers;

// use crate::middleware::auth_user;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;

use crate::middleware::auth_user;

pub fn routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/me", get(handlers::me))
        .route_layer(from_fn_with_state(db.clone(), auth_user))
        .route("/sign_in", post(handlers::sign_in))
        .route("/sign_up", post(handlers::sign_up))
        .route("/refresh", post(handlers::refresh_token))
        .with_state(db)
}
