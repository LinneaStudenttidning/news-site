use std::{env, fs};

use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use crate::database::models::creator::Creator;
use crate::defaults::DATA_DIR;
use crate::error::Error;

fn read_token_secret() -> Vec<u8> {
    dotenv().ok();
    let data_dir = env::var("DATA_DIR").unwrap_or(DATA_DIR.into());
    let token_path = format!("{data_dir}/token_key");
    let token_key = fs::read_to_string(&token_path)
        .unwrap_or_else(|_| panic!("Could not read token key!\nFile: {0}\nGenerate via: mkdir -p {1} && openssl rand -hex 32 > {0}", &token_path, &data_dir));
    token_key.as_bytes().into()
}

pub fn get_encoding_key() -> EncodingKey {
    EncodingKey::from_secret(&read_token_secret())
}
pub fn get_decoding_key() -> DecodingKey {
    DecodingKey::from_secret(&read_token_secret())
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
    type Error = Error;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = request.cookies().get("token").map(|cookie| cookie.value());

        let token = match cookie {
            Some(token) => {
                jsonwebtoken::decode::<Claims>(token, &get_decoding_key(), &Validation::default())
                    .map(|token| token.claims)
            }
            None => {
                return Outcome::Error((
                    Status::BadRequest,
                    Error::create("Claims Guard", "No 'token' cookie!", Status::BadRequest),
                ))
            }
        };

        match token {
            Ok(valid_token) => Outcome::Success(valid_token),
            Err(_) => Outcome::Error((
                Status::Unauthorized,
                Error::create("Claims Guard", "Invalid token!", Status::Unauthorized),
            )),
        }
    }
}
