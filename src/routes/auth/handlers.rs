use crate::{
    models::users::{self, Model as UserModel},
    utils::{
        app_error::AppError,
        jwt::{create_jwt, AuthTokens},
    },
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
    #[schema(example = "John Doe")]
    fullname: String,
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
pub async fn sign_up(
    State(db): State<DatabaseConnection>,
    Json(sign_up_params): Json<SignUpParams>,
) -> Result<Json<AuthTokens>, AppError> {
    let new_user = users::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        fullname: Set(sign_up_params.fullname),
        email: Set(sign_up_params.email),
        encrypted_password: Set(sign_up_params.password),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|_op| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))?;

    let token = create_jwt(new_user.id.unwrap())?;

    Ok(Json(token))
}

#[derive(ToSchema, Serialize, Clone)]
pub struct CurrentUser {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(value_type = String, example = "e15f9d3e-7fe5-4822-9f9d-0d4d4456d33a")]
    pub uuid: Uuid,
    #[schema(example = "John Doe")]
    pub fullname: String,
    #[schema(example = "john_doe@email.com")]
    pub email: String,
    #[schema(example = "2023-11-05T13:15:30Z")]
    pub created_at: String,
    #[schema(example = "2023-11-05T13:15:30Z")]
    pub updated_at: String,
}

#[utoipa::path(
    get,
    tag = "Auth",
    path = "/auth/me",
    responses(
        (status = 200, description = "Current User", body = CurrentUser),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema)
    ),
    security(("bearer_auth" = []))
)]
pub async fn me(Extension(current_user): Extension<UserModel>) -> Result<Json<CurrentUser>, ()> {
    let me = CurrentUser {
        id: current_user.id,
        uuid: current_user.uuid,
        fullname: current_user.fullname,
        email: current_user.email,
        created_at: current_user.created_at.to_string(),
        updated_at: current_user.updated_at.to_string(),
    };

    Ok(Json(me))
}
