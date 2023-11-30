use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Clone)]
#[schema(example = json!({"message": "This is a health check"}))]
pub struct Home {
    message: &'static str,
}

#[utoipa::path(get, tag = "Home", path = "/", responses((status = 200, description = "Health Check", body = Home )))]
pub async fn home() -> Json<Home> {
    Json(Home {
        message: "This is a health check",
    })
}
