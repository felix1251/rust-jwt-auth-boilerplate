use super::app_error::AppError;
use axum::http::StatusCode;

pub fn hash_password(password: String) -> Result<String, AppError> {
    bcrypt::hash(password, 12)
        .map_err(|_err| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|_err| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))
}
