use crate::utils::{
    app_error::AppError,
    jwt::{create_jwt, AuthTokens},
};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SignInParams {
    #[schema(example = "john_doe@email.com")]
    email: String,
    #[schema(example = "password")]
    password: String,
}

#[derive(ToSchema)]
pub struct InvalidCredentials {
    #[schema(example = 400)]
    pub status: u16,
    #[schema(example = "Invalid credentials")]
    pub message: String,
}

#[utoipa::path(
    post,
    request_body = SignInParams,
    tag = "Auth",
    path = "/auth/sign_in",
    responses(
        (status = 200, description = "Token Response", body = AuthTokens),
        (status = 400, description = "Invalid Credentials", body = InvalidCredentials)
    )
)]
pub async fn sign_in(
    Json(_sign_in_params): Json<SignInParams>,
) -> Result<Json<AuthTokens>, AppError> {
    let token = create_jwt(1)?;

    Ok(Json(token))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SignUpParams {
    #[schema(example = "john_doe@email.com")]
    email: String,
    #[schema(example = "password")]
    password: String,
    #[schema(example = "password")]
    password_confirmation: String,
}

#[utoipa::path(
    post,
    request_body = SignUpParams,
    tag = "Auth",
    path = "/auth/sign_up",
    responses(
        (status = 201, description = "User created with token Response", body = AuthTokens),
    )
)]
pub async fn sign_up(Json(_sign_up_params): Json<SignUpParams>) -> Result<(), AppError> {
    Err(AppError::new(StatusCode::BAD_REQUEST, "BAD_REQUEST"))
}
