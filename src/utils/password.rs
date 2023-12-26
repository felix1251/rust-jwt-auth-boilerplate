use super::app_error::AppError;
use axum::http::StatusCode;

pub fn compare_and_hash_password(
    password: String,
    password_confirmation: String,
) -> Result<String, AppError> {
    if !password.ne(&password_confirmation) {
        return Err(AppError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Password does not match",
        ));
    }

    bcrypt::hash(password, 12)
        .map_err(|_err| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|_err| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))
}
