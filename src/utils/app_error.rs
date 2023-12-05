use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse {
                status: self.code.as_u16(),
                error: self.message.clone(),
            }),
        )
            .into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    status: u16,
    error: String,
}
