use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, Error};
use uuid::Uuid;

use crate::database::DatabaseHandler;

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
    }
}
