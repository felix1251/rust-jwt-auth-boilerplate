use crate::models::users::Entity as Users;
use crate::utils::{app_error::AppError, jwt::decode_token};
use axum::extract::State;
use axum::http::HeaderMap;
use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
};
use dotenvy_macro::dotenv;
use sea_orm::{DatabaseConnection, EntityTrait};
use tower_http::cors::{Any, CorsLayer};

pub async fn auth_user(
    State(db): State<DatabaseConnection>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let headers = request.headers();

    let auth_header = get_auth_header(headers)?;
    let token = strip_auth_header(auth_header)?;

    let secret = dotenv!("JWT_TOKEN_SECRET");
    let decoded_token = decode_token(token, secret)?;

    let user = Users::find_by_id(decoded_token.id)
        .one(&db)
        .await
        .map_err(|_op| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))?;

    match user {
        Some(current_user) => {
            request.extensions_mut().insert(current_user);

            Ok(next.run(request).await)
        }
        None => Err(AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED")),
    }
}

fn get_auth_header(headers: &HeaderMap) -> Result<&str, AppError> {
    let auth_header = headers.get("Authorization");

    match auth_header {
        Some(token) => Ok(token.to_str().unwrap()),
        None => Err(AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED")),
    }
}

fn strip_auth_header(auth_header: &str) -> Result<&str, AppError> {
    let token = auth_header.strip_prefix("Bearer ");

    match token {
        Some(token) => Ok(token),
        None => Err(AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED")),
    }
}

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any)
}

pub async fn fallback() -> Result<(), AppError> {
    Err(AppError::new(StatusCode::NOT_FOUND, "ROUTE_NOT_FOUND"))
}
