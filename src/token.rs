use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
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

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Claims {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = request.cookies().get("token").map(|cookie| cookie.value());

        let token = match cookie {
            Some(token) => {
                jsonwebtoken::decode::<Claims>(token, &get_decoding_key(), &Validation::default())
                    .map(|token| token.claims)
            }
            None => return Outcome::Error((Status::BadRequest, "No 'token' cookie!".to_string())),
        };

        match token {
            Ok(valid_token) => Outcome::Success(valid_token),
            Err(_) => Outcome::Error((Status::Unauthorized, "Invalid token!".to_string())),
        }
    }
}
