use crate::utils::{
    app_error::AppError,
    jwt::{create_jwt, AuthTokens},
};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RequestUser {
    #[schema(example = 1)]
    id: i32,
}

#[utoipa::path(
    post,
    request_body = RequestUser,
    tag = "Auth",
    path = "/auth/sign_in",
    responses(
        (status = 200, description = "Token Response", body = AuthTokens),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema)
    )
)]
pub async fn sign_in(Json(request_user): Json<RequestUser>) -> Result<Json<AuthTokens>, AppError> {
    let token = create_jwt(request_user.id)?;

    Ok(Json(token))
}
