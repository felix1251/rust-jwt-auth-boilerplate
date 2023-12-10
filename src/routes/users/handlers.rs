use crate::models::users::Model as UserModel;
use axum::Extension;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

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
}

#[utoipa::path(
    get,
    tag = "User",
    path = "/users/me",
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
    };

    Ok(Json(me))
}
