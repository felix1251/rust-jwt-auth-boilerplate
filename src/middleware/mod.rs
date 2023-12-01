use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use tower_http::cors::{Any, CorsLayer};

use crate::routes::ErrRes;

pub async fn auth_user(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrRes>)> {
    let headers = request.headers();

    let _auth_header = headers
        .get("Authorization")
        .ok_or_else(|| {
            let status = StatusCode::UNAUTHORIZED;
            (
                status,
                Json(ErrRes {
                    status: status.as_u16(),
                    message: "UNAUTHORIZED",
                }),
            )
        })?
        .to_str();

    // some logic here to check if the auth header is a valid JWT token

    Ok(next.run(request).await)
}

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any)
}

pub async fn fallback() -> Result<(), (StatusCode, Json<ErrRes>)> {
    let status = StatusCode::NOT_FOUND;
    Err((
        status,
        Json(ErrRes {
            status: status.as_u16(),
            message: "ROUTE_NOT_FOUND",
        }),
    ))
}
