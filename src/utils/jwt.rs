use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::app_error::AppError;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: i32,    // ID of the user
    pub exp: usize, // Expiration time (as UTC timestamp)
    pub iat: usize, // Issued at (as UTC timestamp)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthTokens {
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.....")]
    token: String,
    #[schema(example = "eyJFsdskDHJiOiJKV1QiLCJhbGudgsgSGIUz.....")]
    refresh_token: String,
}

pub fn create_jwt(id: i32) -> Result<AuthTokens, AppError> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;

    // Token
    let expires_in = now + Duration::hours(3);
    let exp = expires_in.timestamp() as usize;

    let claim = Claims { id, exp, iat };
    let secret = dotenv!("JWT_TOKEN_SECRET");
    let token = encode_token(claim, secret)?;

    // Refresh Token
    let expires_in = now + Duration::days(2);
    let exp = expires_in.timestamp() as usize;

    let claim = Claims { id, exp, iat };
    let secret = dotenv!("JWT_REFRESH_TOKEN_SECRET");
    let refresh_token = encode_token(claim, secret)?;

    Ok(AuthTokens {
        token,
        refresh_token,
    })
}

pub fn encode_token(claim: Claims, secret: &str) -> Result<String, AppError> {
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &claim, &key)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());

    let decoded_token = decode::<Claims>(&token, &key, &Validation::new(Algorithm::HS256))
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED")
            }
            _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        })?;

    Ok(decoded_token.claims)
}
