use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DynamicAppError {
    String(&'static str),
    ValidationErrors(ValidationErrors),
}

pub struct AppError {
    code: StatusCode,
    error: DynamicAppError,
}

impl AppError {
    pub fn new(code: StatusCode, error: impl Into<DynamicAppError>) -> Self {
        Self {
            code,
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse {
                status: self.code.as_u16(),
                error: self.error,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    error: DynamicAppError,
}
