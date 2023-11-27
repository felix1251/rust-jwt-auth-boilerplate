use axum::extract::{Path, Query};
use axum::http::HeaderMap;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TestJson {
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryParams {
    id: i32,
    message: String,
}

// Simulate JSON (GET /examples/test_json)
pub async fn test_json(Json(payload): Json<TestJson>) -> Json<TestJson> {
    Json(payload)
}

// Simulate path variables (POST /examples/path_vars/:id)
pub async fn path_vars(Path(id): Path<i32>) -> String {
    id.to_string()
}

// Simulate query parameters (GET /examples/query_params)
pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}

// Simulate headers (GET /examples/headers)
pub async fn headers(mut headers: HeaderMap) -> HeaderMap {
    headers.insert("x-my-hdr", "abc".parse().unwrap());
    headers
}
