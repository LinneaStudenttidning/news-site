use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::{database::DatabaseHandler, error::Error};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    pub id: Uuid,
    pub author: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub tags: Vec<String>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            author: "UNKNOWN!".into(),
            description: None,
            created_at: Utc::now().naive_utc(),
            tags: Vec::new(),
        }
    }
}

impl Image {
    /// Create a new `Image`; this should be prefered over manually creating a new `Image`.
    pub fn create(author: &str, description: Option<&str>, tags: Vec<&str>) -> Self {
        Self {
            author: author.into(),
            description: description.map(Into::into),
            tags: tags.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Saves an instance of `Image` to the database.
    pub async fn save_to_db(&self, db: &DatabaseHandler) -> Result<PgQueryResult, Error> {
        sqlx::query_file!(
            "sql/images/insert.sql",
            self.id,
            self.author,
            self.description,
            &self.tags
        )
        .execute(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Gets ALL `Image`s from the database with `tag`.
    pub async fn get_by_tag(db: &DatabaseHandler, tag: &str) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/images/get_by_tag.sql", tag)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ALL `Image`s from the database with any of `tags`.
    pub async fn get_by_any_of_tags(
        db: &DatabaseHandler,
        tags: &[String],
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/images/get_by_any_of_tags.sql", tags)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }
}
