use crate::error::Error;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use chrono::{DateTime, Local, Utc};
use jsonwebtoken::Header;
use rocket::http::Status;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
pub struct Creator {
    /// `display_name` may use any characters.
    pub display_name: String,
    /// `username` should be match the regex /[\w\-\.]+/
    pub username: String,
    pub password: String,
    pub biography: String,
    pub joined_at: DateTime<Local>,
    pub role: CreatorRole,
}

impl Default for Creator {
    fn default() -> Self {
        Self {
            display_name: "No Name".to_string(),
            username: "no_name".to_string(),
            password: "".to_string(),
            biography: "Empty biography.".to_string(),
            joined_at: Local::now(),
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

        Ok(true)
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
    pub async fn save_to_db(&self, db: &DatabaseHandler) -> Result<Creator, Error> {
        let user_exists = Self::get_by_username(db, &self.username).await.is_ok();
        if user_exists {
            return Err(Error::create(
                "Creator::save_to_db",
                "User already exists!",
                Status::BadRequest,
            ));
        }

        sqlx::query_file_as!(
            Creator,
            "sql/creators/insert.sql",
            self.display_name,
            self.username,
            self.password,
            self.biography,
            &self.role as &CreatorRole
        )
        .fetch_one(&db.pool)
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

    /// Gets ALL creators from the database which that have at least one published text.
    pub async fn get_all_authors(db: &DatabaseHandler) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/creators/get_all_authors.sql")
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
    ) -> Result<Self, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/creators/update.sql",
            display_name,
            biography,
            username
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
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

    /// Promotes a user to `CreatorRole::Publisher`
    /// FIXME: Return type.
    pub async fn promote(db: &DatabaseHandler, username: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE creators SET role = 'publisher' WHERE username = $1",
            username
        )
        .execute(&db.pool)
        .await
        .map(|_| ())
        .map_err(Error::from)
    }

    /// Demotes a user to `CreatorRole::Writer`
    /// FIXME: Return type.
    pub async fn demote(db: &DatabaseHandler, username: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE creators SET role = 'writer' WHERE username = $1",
            username
        )
        .execute(&db.pool)
        .await
        .map(|_| ())
        .map_err(Error::from)
    }

    /// Demotes a user to `CreatorRole::Writer`
    /// FIXME: Return type.
    pub async fn lock(db: &DatabaseHandler, username: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE creators SET password = 'LOCKED' WHERE username = $1",
            username
        )
        .execute(&db.pool)
        .await
        .map(|_| ())
        .map_err(Error::from)
    }

    /// Change a users password.
    /// * `password` is supposed to **not** be hashed.`
    pub async fn change_password(
        db: &DatabaseHandler,
        username: &str,
        password: &str,
    ) -> Result<(), Error> {
        let creator = Creator::get_by_username(db, username).await?;
        let new_password = Creator::hash_password(password)?;

        sqlx::query!(
            "UPDATE creators SET password = $1 WHERE username = $2",
            creator.username,
            new_password
        )
        .execute(&db.pool)
        .await?;

        Ok(())
    }
}
