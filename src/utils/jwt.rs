use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    id: u32,    // ID of the user
    exp: usize, // Expiration time (as UTC timestamp)
    iat: usize, // Issued at (as UTC timestamp)
}

pub fn create_jwt(id: u32) -> Result<String, AppError> {
    let mut now = Utc::now();

    let iat = now.timestamp() as usize;

    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;

    let claim = UserClaims { id, exp, iat };
    let secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &claim, &key)
        .map_err(|_err| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))
}

// pub fn is_valid() -> Result<bool, StatusCode> {
//     todo!()
// }
