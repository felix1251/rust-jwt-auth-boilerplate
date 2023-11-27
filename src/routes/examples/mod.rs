use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TestJson {
    message: String,
}

// Simulate JSON
pub async fn test_json(Json(body): Json<TestJson>) -> Json<TestJson> {
    Json(body)
}

// Simulate path variables
pub async fn path_vars(Path(id): Path<i32>) -> String {
    id.to_string()
}
