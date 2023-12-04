use super::ErrRes;
use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]

pub struct UserMeSchema {
    #[schema(example = "this is me")]
    message: &'static str,
}

#[utoipa::path(get, tag = "User", path = "/v1/users/me",
    responses(
        (status = 200, description = "Current User", body = UserMeSchema),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema)
    ),
    security(("bearer_auth" = []))
)]
pub async fn me(headers: HeaderMap) -> Result<Json<UserMeSchema>, (StatusCode, Json<ErrRes>)> {
    let _auth_header = headers.get("Authorization").unwrap().to_str();

    Ok(Json(UserMeSchema {
        message: "this is me",
    }))
}
