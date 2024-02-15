use crate::{
    database::{
        mutation::users::{create_user, CreateUserPayload},
        query::users::{find_user_by_email, find_user_by_id},
    },
    middleware::get_auth_token_header,
    models::users::Model as UserModel,
    utils::{
        app_error::{AppError, DynamicAppError},
        jwt::{create_jwt, decode_token, AuthTokens},
        password::verify_password,
    },
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    Extension, Json,
};
use dotenvy_macro::dotenv;
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct SignInParams {
    #[schema(example = "john_doe@email.com", nullable = false, required = true)]
    #[validate(
        required(message = "Email is required"),
        email(message = "Invalid Email")
    )]
    email: Option<String>,
    #[schema(example = "password", nullable = false, required = true)]
    #[validate(required(message = "Password is required"))]
    password: Option<String>,
}

/// User Sign In
#[utoipa::path(
    post,
    request_body = SignInParams,
    tag = "Auth",
    path = "/auth/sign_in",
    responses(
        (status = 200, description = "Token response", body = AuthTokens),
        (status = 404, description = "Invalid credentials", body = InvalidCredentialSchema),
        (status = 422, description = "Request body validation errors", body = ValidationErrorSchema),
        (status = 500, description = "Internal server error", body = InternalErrorSchema),
    )
)]
pub async fn sign_in(
    State(db): State<DatabaseConnection>,
    Json(params): Json<SignInParams>,
) -> Result<Json<AuthTokens>, AppError> {
    if let Err(err) = params.validate() {
        return Err(AppError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            DynamicAppError::ValidationErrors(err),
        ));
    }

    let db_user = find_user_by_email(params.email.unwrap(), db).await?;

    if let Some(user) = db_user {
        if !verify_password(params.password.unwrap(), &user.encrypted_password)? {
            return Err(AppError::new(
                StatusCode::NOT_FOUND,
                DynamicAppError::String("INVALID_CREDENTIALS"),
            ));
        }
        let token = create_jwt(user.id)?;
        return Ok(Json(token));
    }
    return Err(AppError::new(
        StatusCode::NOT_FOUND,
        DynamicAppError::String("INVALID_CREDENTIALS"),
    ));
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct SignUpParams {
    #[schema(example = "john_doe@email.com", nullable = false, required = true)]
    #[validate(
        required(message = "Email is required"),
        email(message = "Invalid Email")
    )]
    pub email: Option<String>,
    #[schema(example = "John Doe", nullable = false, required = true)]
    #[validate(required(message = "Fullname is required"))]
    pub fullname: Option<String>,
    #[schema(example = "password", nullable = false, required = true)]
    #[validate(
        required(message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: Option<String>,
}

/// User Sign Up
#[utoipa::path(
    post,
    request_body = SignUpParams,
    tag = "Auth",
    path = "/auth/sign_up",
    responses(
        (status = 201, description = "User created with token response", body = AuthTokens),
        (status = 409, description = "User exist or Invalid", body = UserExistOrInvalidSchema),
        (status = 422, description = "Request body validation errors", body = ValidationErrorSchema),
        (status = 500, description = "Internal server error", body = InternalErrorSchema),
    )
)]
pub async fn sign_up(
    State(db): State<DatabaseConnection>,
    Json(params): Json<SignUpParams>,
) -> Result<Json<AuthTokens>, AppError> {
    if let Err(err) = params.validate() {
        return Err(AppError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            DynamicAppError::ValidationErrors(err),
        ));
    }

    let create_user_payload = CreateUserPayload {
        fullname: params.fullname.unwrap(),
        email: params.email.unwrap(),
        password: params.password.unwrap(),
    };
    let new_user = create_user(create_user_payload, db).await?;
    let token = create_jwt(new_user.id.unwrap())?;

    return Ok(Json(token));
}

/// Refresh token
#[utoipa::path(
    post,
    tag = "Auth",
    path = "/auth/refresh",
    responses(
        (status = 201, description = "Refresh token response", body = AuthTokens),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema),
        (status = 500, description = "Internal Server Error", body = InternalErrorSchema),
    ),
    security(("bearer_auth" = []))
)]
pub async fn refresh_token(
    State(db): State<DatabaseConnection>,
    request: Request,
) -> Result<Json<AuthTokens>, AppError> {
    let token = get_auth_token_header(request.headers())?;
    let secret = format!("{}", dotenv!("JWT_REFRESH_TOKEN_SECRET"));
    let decoded_refresh_token = decode_token(token, secret)?;

    // find if user exist on db
    let db_user = find_user_by_id(decoded_refresh_token.id, db).await?;

    if let Some(user) = db_user {
        let token = create_jwt(user.id)?;

        return Ok(Json(token));
    }

    return Err(AppError::new(
        StatusCode::UNAUTHORIZED,
        DynamicAppError::String("UNAUTHORIZED"),
    ));
}

#[derive(ToSchema, Serialize, Deserialize, Clone)]
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

/// Current user information
#[utoipa::path(
    get,
    tag = "Auth",
    path = "/auth/me",
    responses(
        (status = 200, description = "Current user info", body = CurrentUser),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema),
        (status = 500, description = "Internal server error", body = InternalErrorSchema),
    ),
    security(("bearer_auth" = []))
)]
pub async fn me(Extension(current_user): Extension<UserModel>) -> Json<CurrentUser> {
    let me = CurrentUser {
        uuid: current_user.uuid,
        fullname: current_user.fullname,
        email: current_user.email,
        created_at: current_user.created_at,
        updated_at: current_user.updated_at,
    };

    return Json(me);
}
