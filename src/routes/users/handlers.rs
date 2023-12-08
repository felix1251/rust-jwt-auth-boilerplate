use crate::models::users;
use crate::utils::{
    app_error::AppError,
    jwt::{create_jwt, Tokens},
};
use axum::Extension;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize, Clone)]
pub struct CurrentUser {
    #[schema(example = 1)]
    pub id: i32,
    pub uuid: Uuid,
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
        (status = 200, description = "Current User", body = CurrentUser),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema)
    ),
    security(("bearer_auth" = []))
)]
pub async fn me(Extension(current_user): Extension<users::Model>) -> Result<Json<CurrentUser>, ()> {
    let me = CurrentUser {
        id: current_user.id,
        uuid: current_user.uuid,
        fullname: current_user.fullname,
        email: current_user.email,
    };

    Ok(Json(me))
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
