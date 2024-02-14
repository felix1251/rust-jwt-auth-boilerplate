use crate::database::query::users::find_user_by_id;
use crate::utils::app_error::DynamicErrorType;
use crate::utils::{app_error::AppError, jwt::decode_token};
use axum::http::HeaderMap;
use axum::{
    extract::Request,
    extract::State,
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use dotenvy_macro::dotenv;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

pub async fn auth_user(
    State(db): State<DatabaseConnection>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // let auth_header = get_auth_header(headers)?;
    let token = get_auth_token_header(request.headers())?;
    let secret = format!("{}", dotenv!("JWT_TOKEN_SECRET"));
    let decoded_token = decode_token(token.clone(), secret)?;
    let user = find_user_by_id(decoded_token.id, db).await?;

    if let Some(current_user) = user {
        request.extensions_mut().insert(current_user);
        return Ok(next.run(request).await);
    }

    return Err(AppError::new(
        StatusCode::UNAUTHORIZED,
        DynamicErrorType::String("UNAUTHORIZED".to_string()),
    ));
}

pub fn get_auth_token_header(headers: &HeaderMap) -> Result<String, AppError> {
    let token = headers
        .typed_get::<Authorization<Bearer>>()
        .ok_or(AppError::new(
            StatusCode::UNAUTHORIZED,
            DynamicErrorType::String("UNAUTHORIZED".to_string()),
        ))?
        .token()
        .to_owned();

    Ok(token)
}

// pub fn strip_auth_header(auth_header: &str) -> Result<&str, AppError> {
//     let token = auth_header.strip_prefix("Bearer ");
//     if let Some(token) = token {
//         return Ok(token);
//     }
//     return Err(AppError::new(
//         StatusCode::UNAUTHORIZED,
//         DynamicErrorType::String("UNAUTHORIZED".to_string()),
//     ));
// }

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
}

pub async fn fallback() -> Result<(), AppError> {
    return Err(AppError::new(
        StatusCode::NOT_FOUND,
        DynamicErrorType::String("UNAUTHORIZED".to_string()),
    ));
}
