use crate::{
    models::users::{self, Entity as Users, Model as UserModel},
    utils::{
        app_error::AppError,
        jwt::{create_jwt, AuthTokens},
        password::{hash_password, verify_password},
    },
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct SignInParams {
    #[schema(example = "john_doe@email.com")]
    #[validate(email)]
    email: String,
    #[schema(example = "password")]
    password: String,
}

#[derive(ToSchema)]
pub struct InvalidCredentials {
    #[schema(example = 404)]
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
        (status = 404, description = "Invalid Credentials", body = InvalidCredentials)
    )
)]
pub async fn sign_in(
    State(db): State<DatabaseConnection>,
    Json(sign_in_params): Json<SignInParams>,
) -> Result<Json<AuthTokens>, AppError> {
    if let Err(err) = sign_in_params.validate() {
        return Err(AppError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("{}", err),
        ));
    }

    let db_user = Users::find()
        .filter(users::Column::Email.eq(sign_in_params.email))
        .one(&db)
        .await
        .map_err(|_err| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
        })?;

    if let Some(user) = db_user {
        if !verify_password(sign_in_params.password, &user.encrypted_password)? {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Invalid Credentials"));
        }
        let token = create_jwt(user.id)?;
        return Ok(Json(token));
    }

    Err(AppError::new(StatusCode::NOT_FOUND, "Invalid Credentials"))
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct SignUpParams {
    #[schema(example = "john_doe@email.com")]
    #[validate(email)]
    email: String,
    #[schema(example = "John Doe")]
    fullname: String,
    #[schema(example = "password")]
    #[validate(length(min = 6))]
    password: String,
    #[schema(example = "password")]
    #[validate(must_match = "password")]
    password_confirmation: String,
}

#[utoipa::path(
    post,
    request_body = SignUpParams,
    tag = "Auth",
    path = "/auth/sign_up",
    responses(
        (status = 201, description = "User created with token Response", body = AuthTokens),
        (status = 422, description = "Validation Errors", body = ValidationErrorSchema),
    )
)]
pub async fn sign_up(
    State(db): State<DatabaseConnection>,
    Json(sign_up_params): Json<SignUpParams>,
) -> Result<Json<AuthTokens>, AppError> {
    if let Err(err) = sign_up_params.validate() {
        return Err(AppError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("{}", err),
        ));
    }

    let new_user = users::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        fullname: Set(sign_up_params.fullname),
        email: Set(sign_up_params.email),
        encrypted_password: Set(hash_password(sign_up_params.password)?),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|err| match err {
        sea_orm::DbErr::Query(_err) => AppError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "User does not exist or Invalid",
        ),
        _else => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
    })?;

    let token = create_jwt(new_user.id.unwrap())?;
    Ok(Json(token))
}

#[derive(ToSchema, Serialize, Clone)]
pub struct CurrentUser {
    #[schema(value_type = String, example = "e15f9d3e-7fe5-4822-9f9d-0d4d4456d33a")]
    pub uuid: Uuid,
    #[schema(example = "John Doe")]
    pub fullname: String,
    #[schema(example = "john_doe@email.com")]
    pub email: String,
    #[schema(value_type = String, example = "2023-11-05T13:15:30Z")]
    pub created_at: DateTimeWithTimeZone,
    #[schema(value_type = String, example = "2023-11-05T13:15:30Z")]
    pub updated_at: DateTimeWithTimeZone,
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
        uuid: current_user.uuid,
        fullname: current_user.fullname,
        email: current_user.email,
        created_at: current_user.created_at,
        updated_at: current_user.updated_at,
    };

    Ok(Json(me))
}
