use super::ErrRes;
use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
#[schema(example = json!({"message": "hi"}))]
pub struct UserMeSchema {
    message: String,
}

#[utoipa::path(
    get, tag = "User",
    path = "/v1/users/me",
    responses(
        (status = 200, description = "Current User", body = UserMeSchema ),
        (status = 401, description = "Unauthenticated", body = UnauthorizedSchema )
    )
)]
pub async fn me(headers: HeaderMap) -> Result<Json<UserMeSchema>, (StatusCode, Json<ErrRes>)> {
    let _auth_header = headers.get("Authorization").unwrap().to_str();

    Ok(Json(UserMeSchema {
        message: "hi".to_owned(),
    }))
}
