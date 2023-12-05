use crate::utils::app_error::AppError;
use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_http::cors::{Any, CorsLayer};

pub async fn auth_user(request: Request, next: Next) -> Result<Response, AppError> {
    let headers = request.headers();

    let _auth_header = headers
        .get("Authorization")
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .to_str();

    // some logic here to check if the auth header is a valid JWT token

    Ok(next.run(request).await)
}

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any)
}

pub async fn fallback() -> Result<(), AppError> {
    Err(AppError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))
}
