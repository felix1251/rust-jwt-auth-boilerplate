use crate::models::users::Entity as Users;
use crate::routes::users::CurrentUser;
use crate::utils::{app_error::AppError, jwt::decode_token};
use axum::extract::State;
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

    let auth_header = headers
        .get("Authorization")
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .to_str()
        .unwrap();

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?;

    let secret = dotenv!("JWT_TOKEN_SECRET");
    let decoded_token = decode_token(token, secret)?;

    let user = Users::find_by_id(decoded_token.id)
        .one(&db)
        .await
        .unwrap()
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?;

    let current_user = CurrentUser {
        id: user.id,
        uuid: user.uuid,
        fullname: user.fullname,
        email: user.email,
    };
    request.extensions_mut().insert(current_user);

    Ok(next.run(request).await).into()
}

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any)
}

pub async fn fallback() -> Result<(), AppError> {
    Err(AppError::new(StatusCode::NOT_FOUND, "ROUTE_NOT_FOUND"))
}
