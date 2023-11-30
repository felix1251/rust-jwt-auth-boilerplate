use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Clone)]
#[schema(example = json!({"message": "This is a health check"}))]
pub struct HomeRes {}

#[utoipa::path(get, path = "/", responses((status = 200, description = "Health Check", body = HomeRes )))]
pub async fn home() -> Json<Value> {
    Json(json!({"message": "This is a health check"}))
}
