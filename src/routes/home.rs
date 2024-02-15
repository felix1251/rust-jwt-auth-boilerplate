use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct HomeSchema {
    #[schema(example = "This is a health check")]
    message: &'static str,
}

/// Health check
#[utoipa::path(
    get,
    tag = "Home",
    path = "/",
    responses(
        (status = 200, description = "Health Check", body = HomeSchema )
    )
)]
pub async fn home() -> Json<HomeSchema> {
    Json(HomeSchema {
        message: "This is a health check",
    })
}
