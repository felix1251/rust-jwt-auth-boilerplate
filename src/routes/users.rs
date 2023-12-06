use crate::utils::{app_error::AppError, jwt::create_jwt};

use axum::{http::HeaderMap, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]

pub struct UserMeSchema {
    #[schema(example = "this is me")]
    message: &'static str,
}

#[utoipa::path(
    get,
    tag = "User",
    path = "/v1/users/me",
    responses(
        (status = 200, description = "Current User", body = UserMeSchema),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema)
    ),
    security(("bearer_auth" = []))
)]
pub async fn me(headers: HeaderMap) -> Result<Json<UserMeSchema>, AppError> {
    let _auth_header = headers.get("Authorization").unwrap().to_str();

    Ok(Json(UserMeSchema {
        message: "this is me",
    }))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RequestUser {
    #[schema(example = 1)]
    id: u32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResponseUser {
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.....")]
    token: String,
}

#[utoipa::path(
    post,
    request_body = RequestUser,
    tag = "User",
    path = "/v1/users/sign_in",
    responses(
        (status = 200, description = "Token Response", body = ResponseUser),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema)
    )
)]
pub async fn sign_in(
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, AppError> {
    let token = create_jwt(request_user.id)?;

    Ok(Json(ResponseUser { token }))
}
