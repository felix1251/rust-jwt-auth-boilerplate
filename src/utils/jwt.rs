use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: String,
    exp: usize,
    iat: usize,
}

// pub fn create() -> Result<String, StatusCode> {}

// pub fn is_valid() -> Result<bool, StatusCode> {
//     todo!()
// }
