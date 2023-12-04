use super::ErrRes;
use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};

pub async fn me(headers: HeaderMap) -> Result<String, (StatusCode, Json<ErrRes>)> {
    let _auth_header = headers.get("Authorization").unwrap().to_str();

    Ok("ME".to_owned())
}
