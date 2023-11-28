// use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    user_id: String, // ID of the user
    exp: usize,      // Expiration time (as UTC timestamp
    iat: usize,      // Issued at (as UTC timestamp)
}

// pub fn create() -> Result<String, StatusCode> {}

// pub fn is_valid() -> Result<bool, StatusCode> {
//     todo!()
// }
