use chrono::{NaiveDateTime, Utc};
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
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
    pub text_type: TextType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
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

    pub async fn get_by_id(db: &DatabaseHandler, id: i32) -> Result<Self, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_id.sql", id)
            .fetch_one(&db.pool)
            .await
    }

    pub async fn get_by_author(db: &DatabaseHandler, author: &str) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_author.sql", author)
            .fetch_all(&db.pool)
            .await
    }

    pub async fn get_by_type(
        db: &DatabaseHandler,
        text_type: TextType,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_type.sql", text_type as TextType)
            .fetch_all(&db.pool)
            .await
    }

    pub async fn search(db: &DatabaseHandler) -> Result<Vec<Self>, Error> {
        println!("{:?}", &db.pool);
        todo!("IMPLEMENT ME!")
    }
}
