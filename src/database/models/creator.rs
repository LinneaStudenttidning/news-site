use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use chrono::NaiveDateTime;
use chrono::Utc;
use jsonwebtoken::Header;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

use crate::database::DatabaseHandler;
use crate::token::get_encoding_key;
use crate::token::Claims;

type HashError = argon2::password_hash::Error;

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
    fn hash_password(password: &str) -> Result<String, HashError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(hashed_password)
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
    ) -> Result<Self, HashError> {
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
    }

    /// Gets ALL creators from the database.
    pub async fn get_all(db: &DatabaseHandler) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/creators/get_all.sql")
            .fetch_all(&db.pool)
            .await
    }

    /// Gets ONE creator from the database by its `username`.
    pub async fn get_by_username(db: &DatabaseHandler, username: &str) -> Result<Self, Error> {
        sqlx::query_file_as!(Self, "sql/creators/get_by_username.sql", username)
            .fetch_one(&db.pool)
            .await
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

    pub async fn login(&self, password: &str) -> Result<String, String> {
        let password_hash =
            PasswordHash::new(&self.password).map_err(|_| "Error parsing password hash!")?;

        if password_hash.to_string() != password {
            return Err("Invalid password!".into());
        }

        let claims = Claims {
            // FIXME: This should probably be something like 4 hours into the future...
            exp: 0,
            sub: self.username.clone(),
            admin: self.is_publisher(),
            data: self.clone(),
        };

        jsonwebtoken::encode::<Claims>(&Header::default(), &claims, &get_encoding_key())
            .map_err(|_| "Failed to encode token!".into())
    }
}

