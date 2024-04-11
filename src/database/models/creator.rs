use crate::error::Error;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use chrono::NaiveDateTime;
use chrono::Utc;
use jsonwebtoken::Header;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;

use crate::database::DatabaseHandler;
use crate::token::get_encoding_key;
use crate::token::Claims;

const FOUR_HOURS_AS_SECS: usize = 60 * 60 * 4;

/// The type of creator.
/// `Writer` is a "normal" creator, while `Publisher` is more like an admin.
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "creator_role", rename_all = "lowercase")]
pub enum CreatorRole {
    Publisher,
    Writer,
}

/// A `Creator` is someone who can write articles on the site.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creator {
    /// `display_name` may use any characters.
    pub display_name: String,
    /// `username` should be match the regex /[\w\-\.]+/
    pub username: String,
    pub password: String,
    pub biography: String,
    pub joined_at: NaiveDateTime,
    pub role: CreatorRole,
}

impl Default for Creator {
    fn default() -> Self {
        Self {
            display_name: "No Name".to_string(),
            username: "no_name".to_string(),
            password: "".to_string(),
            biography: "Empty biography.".to_string(),
            joined_at: Utc::now().naive_utc(),
            role: CreatorRole::Writer,
        }
    }
}

impl Creator {
    pub fn hash_password(password: &str) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(hashed_password)
    }

    pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, Error> {
        let argon2 = Argon2::default();

        let password_parsed =
            PasswordHash::parse(password_hash, argon2::password_hash::Encoding::default())?;

        if (argon2.verify_password(password.as_bytes(), &password_parsed)).is_err() {
            return Ok(false);
        }

        return Ok(true);
    }

    /// Create a new `Creator` that is a regular Publisher; this should be prefered over manually creating a new `Creator`.
    /// * `username` should never change.
    /// * `display_name` can be changed.
    /// * `password` is automatically hashed.
    /// * `as_publisher` creates user as publisher if true.
    pub fn create(
        username: &str,
        display_name: &str,
        password: &str,
        as_publisher: bool,
    ) -> Result<Self, Error> {
        Ok(Self {
            username: username.to_string(),
            display_name: display_name.to_string(),
            password: Self::hash_password(password)?,
            role: match as_publisher {
                true => CreatorRole::Publisher,
                _ => CreatorRole::Writer,
            },
            ..Default::default()
        })
    }

    /// Checks what it says.
    pub fn is_publisher(&self) -> bool {
        matches!(self.role, CreatorRole::Publisher)
    }

    /// Saves an instance of `Creator` to the database.
    pub async fn save_to_db(&self, db: &DatabaseHandler) -> Result<PgQueryResult, Error> {
        sqlx::query_file!(
            "sql/creators/insert.sql",
            self.display_name,
            self.username,
            self.password,
            self.biography,
            &self.role as &CreatorRole
        )
        .execute(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Gets ALL creators from the database.
    pub async fn get_all(db: &DatabaseHandler) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/creators/get_all.sql")
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ONE creator from the database by its `username`.
    pub async fn get_by_username(db: &DatabaseHandler, username: &str) -> Result<Self, Error> {
        sqlx::query_file_as!(Self, "sql/creators/get_by_username.sql", username)
            .fetch_one(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Updates ONE creator from the data by its `username`.
    pub async fn update_by_username(
        db: &DatabaseHandler,
        username: &str,
        display_name: &str,
        biography: &str,
        password: &str,
    ) -> Result<Self, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/creators/update.sql",
            display_name,
            biography,
            password,
            username
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Changes the display name of a `Creator`.
    pub async fn change_display_name(
        &self,
        db: &DatabaseHandler,
        new_display_name: &str,
    ) -> Result<Self, Error> {
        sqlx::query!(
            "UPDATE creators SET display_name = $1 WHERE username = $2",
            new_display_name,
            &self.username
        )
        .execute(&db.pool)
        .await?;

        // Return what the new `Creator` looks like.
        // FIXME: Maybe this should only return the new `display_name`?
        Ok(Self {
            username: self.username.to_string(),
            password: self.password.to_string(),
            display_name: new_display_name.into(),
            biography: self.biography.to_string(),
            joined_at: self.joined_at,
            role: self.role.clone(),
        })
    }

    pub async fn login(&self, password: &str) -> Result<String, Error> {
        if !Creator::verify_password(password, &self.password)? {
            return Err(Error::create(
                "Password check",
                "Invalid password or problem checking password!",
                Status::BadRequest,
            ));
        }

        let claims = Claims {
            exp: Utc::now().timestamp() as usize + FOUR_HOURS_AS_SECS,
            sub: self.username.clone(),
            admin: self.is_publisher(),
            data: self.clone(),
        };

        jsonwebtoken::encode::<Claims>(&Header::default(), &claims, &get_encoding_key()).map_err(
            |_| {
                Error::create(
                    "Token creation",
                    "Failed to encode token!",
                    Status::InternalServerError,
                )
            },
        )
    }
}
