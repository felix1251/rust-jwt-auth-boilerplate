use super::ErrRes;
use axum::{
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    Json,
};
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
pub async fn headers(headers: HeaderMap) -> Result<String, (StatusCode, Json<ErrRes>)> {
    let header = headers
        .get("x-my-hdr")
        .ok_or_else(|| {
            let status = StatusCode::BAD_REQUEST;
            (
                status,
                Json(ErrRes {
                    status: status.as_u16(),
                    message: "UNAUTHORIZED",
                }),
            )
        })?
        .to_str();

    Ok(header.unwrap().to_owned())
}
