use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::app_error::AppError;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: u32,    // ID of the user
    exp: usize, // Expiration time (as UTC timestamp)
    iat: usize, // Issued at (as UTC timestamp)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Tokens {
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.....")]
    token: String,
    #[schema(example = "eyJFsdskDHJiOiJKV1QiLCJhbGudgsgSGIUz.....")]
    refresh_token: String,
}

pub fn create_jwt(id: u32) -> Result<Tokens, AppError> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;

    // Token
    let expires_in = now + Duration::seconds(30);
    let exp = expires_in.timestamp() as usize;

    let claim = Claims { id, exp, iat };
    let secret = dotenv!("JWT_TOKEN_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    let token = encode_token(claim, key)?;

    // Refresh Token
    let expires_in = now + Duration::days(2);
    let exp = expires_in.timestamp() as usize;

    let claim = Claims { id, exp, iat };
    let secret = dotenv!("JWT_TOKEN_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    let refresh_token = encode_token(claim, key)?;

    Ok(Tokens {
        token,
        refresh_token,
    })
}

fn encode_token(claim: Claims, key: EncodingKey) -> Result<String, AppError> {
    encode(&Header::default(), &claim, &key)
        .map_err(|_err| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))
}

// pub fn is_valid() -> Result<bool, StatusCode> {
//     todo!()
// }
