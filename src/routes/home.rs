use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
#[schema(example = json!({"message": "This is a health check"}))]
pub struct HomeSchema {
    message: &'static str,
}

#[utoipa::path(get, tag = "Home", path = "/", responses((status = 200, description = "Health Check", body = HomeSchema )))]
pub async fn home() -> Json<HomeSchema> {
    Json(HomeSchema {
        message: "This is a health check",
    })
}
