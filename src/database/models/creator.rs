use chrono::NaiveDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

use crate::database::DatabaseHandler;

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
    /// Create a new `Creator` that is a regular Writer; this should be prefered over manually creating a new `Creator`.
    pub fn create_writer(username: &str, display_name: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            display_name: display_name.to_string(),
            password: password.to_string(),
            ..Default::default()
        }
    }

    /// Create a new `Creator` that is a regular Publisher; this should be prefered over manually creating a new `Creator`.
    pub fn create_publisher(username: &str, display_name: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            display_name: display_name.to_string(),
            password: password.to_string(),
            role: CreatorRole::Publisher,
            ..Default::default()
        }
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
}
