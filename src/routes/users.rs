use crate::utils::{
    app_error::AppError,
    jwt::{create_jwt, Tokens},
};
use axum::Extension;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Clone)]

pub struct ResponseUser {
    #[schema(example = 1)]
    pub id: i32,
    pub uuid: String,
    #[schema(example = "John Doe")]
    pub fullname: String,
    #[schema(example = "john_doe@email.com")]
    pub email: String,
}

#[utoipa::path(
    get,
    tag = "User",
    path = "/v1/users/me",
    responses(
        (status = 200, description = "Current User", body = ResponseUser),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema)
    ),
    security(("bearer_auth" = []))
)]
pub async fn me(
    Extension(current_user): Extension<ResponseUser>,
) -> Result<Json<ResponseUser>, AppError> {
    Ok(Json(current_user))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RequestUser {
    #[schema(example = 1)]
    id: i32,
}

#[utoipa::path(
    post,
    request_body = RequestUser,
    tag = "User",
    path = "/v1/users/sign_in",
    responses(
        (status = 200, description = "Token Response", body = Tokens),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema)
    )
)]
pub async fn sign_in(Json(request_user): Json<RequestUser>) -> Result<Json<Tokens>, AppError> {
    let token = create_jwt(request_user.id)?;

    Ok(Json(token))
}
