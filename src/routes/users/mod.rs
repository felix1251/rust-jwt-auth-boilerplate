// pub mod handlers;

// use crate::middleware::auth_user;
// use axum::{middleware::from_fn_with_state, Router};
// use sea_orm::DatabaseConnection;

// pub fn routes(db: DatabaseConnection) -> Router {
//     Router::new()
//         .route_layer(from_fn_with_state(db.clone(), auth_user))
//         .with_state(db)
// }
