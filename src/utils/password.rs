use super::app_error::{AppError, DynamicErrorType};
use axum::http::StatusCode;

pub fn hash_password(password: String) -> Result<String, AppError> {
    bcrypt::hash(password, 7).map_err(|_err| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            DynamicErrorType::String("INTERNAL_SERVER_ERROR".to_string()),
        )
    })
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash).map_err(|_err| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            DynamicErrorType::String("INTERNAL_SERVER_ERROR".to_string()),
        )
    })
}
