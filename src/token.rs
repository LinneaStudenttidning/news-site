use std::{env, fs};

use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use crate::database::models::creator::Creator;
use crate::database::DatabaseHandler;
use crate::defaults::DATA_DIR;
use crate::error::Error;

/// Reads a token from a token secret file.
/// If the token secret file does not exist, a command to generate it will be suggested.
/// This file is expected to be generated with the command suggested, or similar.
fn read_token_secret() -> Vec<u8> {
    dotenv().ok();
    let data_dir = env::var("DATA_DIR").unwrap_or(DATA_DIR.into());
    let token_path = format!("{data_dir}/token_key");
    let token_key = fs::read_to_string(&token_path)
        .unwrap_or_else(|_| panic!("Could not read token key!\nFile: {0}\nGenerate via: mkdir -p {1} && openssl rand -hex 32 > {0}", &token_path, &data_dir));
    token_key.as_bytes().into()
}

/// Gets the `EncodingKey` from the token secret.
pub fn get_encoding_key() -> EncodingKey {
    EncodingKey::from_secret(&read_token_secret())
}

/// Gets the `DecodingKey` from the token secret.
pub fn get_decoding_key() -> DecodingKey {
    DecodingKey::from_secret(&read_token_secret())
}

/// `Claims` is basically the payload for the JWTs.
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
                    Status::Unauthorized,
                    Error::create("Claims Guard", "No 'token' cookie!", Status::BadRequest),
                ))
            }
        };

        let claims = match token {
            Ok(valid_token) => valid_token,
            Err(_) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    Error::create("Claims Guard", "Invalid token!", Status::Unauthorized),
                ))
            }
        };

        let db = match request.rocket().state::<DatabaseHandler>() {
            Some(db) => db,
            None => {
                return Outcome::Error((
                    Status::InternalServerError,
                    Error::create(
                        "Claims Guard",
                        "Could not connect to database!",
                        Status::InternalServerError,
                    ),
                ))
            }
        };

        let creator = match Creator::get_by_username(db, &claims.sub).await {
            Ok(creator) => creator,
            Err(e) => {
                return Outcome::Error((
                    Status::InternalServerError,
                    Error::create("Claims Guard", &e.to_string(), Status::InternalServerError),
                ))
            }
        };

        // This check is performed so that an old (but not expired)
        // token is invalidated on password change.
        if claims.data.password != creator.password {
            return Outcome::Error((
                Status::Unauthorized,
                Error::create(
                    "Claims Guard",
                    "Password hashes do not match!",
                    Status::Unauthorized,
                ),
            ));
        }

        // This check is performed so that an old (but not expired)
        // token is invalidated on role change.
        if claims.admin != creator.is_publisher() {
            return Outcome::Error((
                Status::Unauthorized,
                Error::create("Claims Guard", "Role has changed!", Status::Unauthorized),
            ));
        }

        Outcome::Success(claims)
    }
}
