use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};

pub async fn auth_user(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let headers = request.headers();

    let _auth_header = headers
        .get("Authorization")
        .ok_or_else(|| {
            let status = StatusCode::UNAUTHORIZED;
            (
                status,
                Json(json!({
                    "message": "UNAUTHORIZED",
                    "status": status.as_u16()
                })),
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

pub async fn fallback() -> Result<(), (StatusCode, Json<Value>)> {
    let status = StatusCode::NOT_FOUND;
    Err((
        status,
        Json(json!({
            "message": "ROUTE_NOT_FOUND",
            "status": status.as_u16()}
        )),
    ))
}
