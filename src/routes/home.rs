use crate::swagger::schemas::home::HomeSchema;
use axum::Json;

#[utoipa::path(get, tag = "Home", path = "/", responses((status = 200, description = "Health Check", body = HomeSchema )))]
pub async fn home() -> Json<HomeSchema> {
    Json(HomeSchema {
        message: "This is a health check",
    })
}
