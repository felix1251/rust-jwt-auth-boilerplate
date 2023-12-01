use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Clone)]
#[schema(example = json!({"message": "This is a health check"}))]
pub struct HomeSchema {
    pub message: &'static str,
}
