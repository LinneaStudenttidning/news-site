use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::database::models::creator::Creator;

// FIXME: THSE MUST BE SET BETTER THAN THIS! THIS IS NOT SAFE.
pub const SECRET: &[u8] = "SECRET".as_bytes();
pub fn get_encoding_key() -> EncodingKey {
    EncodingKey::from_secret(SECRET)
}
pub fn get_decoding_key() -> DecodingKey {
    DecodingKey::from_secret(SECRET)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
    pub data: Creator,
    pub admin: bool,
}

