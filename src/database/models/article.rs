use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{self, postgres::PgQueryResult, Error};

use crate::database::DatabaseHandler;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "text_type", rename_all = "lowercase")]
pub enum TextType {
    Article,
    Coverage,
    Opinion,
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Text {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub content: String,
    pub text_type: TextType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            id: 0,
            title: "Missing title!".into(),
            author: "NULL".into(),
            content: "Missing content!".into(),
            text_type: TextType::Other,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: Vec::new(),
        }
    }
}

impl Text {
    pub fn create(
        title: &str,
        author: &str,
        content: &str,
        text_type: TextType,
        tags: Vec<String>,
    ) -> Self {
        Self {
            title: title.into(),
            author: author.into(),
            content: content.into(),
            text_type,
            tags,
            ..Default::default()
        }
    }

    pub async fn save_to_db(&self, db: &DatabaseHandler) -> Result<PgQueryResult, Error> {
        sqlx::query_file!(
            "sql/articles/insert.sql",
            self.title,
            self.author,
            self.content,
            &self.text_type as &TextType,
            &self.tags
        )
        .execute(&db.pool)
        .await
    }
}
