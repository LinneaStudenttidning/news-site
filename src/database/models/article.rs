use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    self,
    postgres::{PgQueryResult, PgRow},
    Error,
};

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
    /// Create a new `Text`; this should be prefered over manually creating a new `Text`.
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

    /// Saves an instance of `Text` to the database.
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

    /// Gets ONE `Text` from the database by its id.
    pub async fn get_by_id(db: &DatabaseHandler, id: i32) -> Result<Self, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_id.sql", id)
            .fetch_one(&db.pool)
            .await
    }

    /// Gets ALL `Text`s from the database by `author`.
    pub async fn get_by_author(db: &DatabaseHandler, author: &str) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_author.sql", author)
            .fetch_all(&db.pool)
            .await
    }

    /// Gets ALL `Text`s from the database by `type`.
    pub async fn get_by_type(
        db: &DatabaseHandler,
        text_type: TextType,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_type.sql", text_type as TextType)
            .fetch_all(&db.pool)
            .await
    }

    /// Gets ALL `Text`s from the database with `tag`.
    pub async fn get_by_tag(db: &DatabaseHandler, tag: &str) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_tag.sql", tag)
            .fetch_all(&db.pool)
            .await
    }

    /// Gets ALL `Text`s from the database with any of `tags`.
    pub async fn get_by_any_of_tags(
        db: &DatabaseHandler,
        tags: &[String],
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_any_of_tags.sql", tags)
            .fetch_all(&db.pool)
            .await
    }

    /// Gets ALL `Text`s from the database matching the search query.
    pub async fn search(db: &DatabaseHandler, query: &str) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/search.sql", query)
            .fetch_all(&db.pool)
            .await
    }

    /// Publishes a `Text`.
    pub async fn publish(db: &DatabaseHandler, id: i32) -> Result<Vec<PgRow>, Error> {
        sqlx::query!("UPDATE articles SET is_published = true WHERE id = $1", id)
            .fetch_all(&db.pool)
            .await
    }

    /// Unpublishes a `Text`.
    pub async fn unpublish(db: &DatabaseHandler, id: i32) -> Result<Vec<PgRow>, Error> {
        sqlx::query!("UPDATE articles SET is_published = false WHERE id = $1", id)
            .fetch_all(&db.pool)
            .await
    }

    /// Deletes ONE `Text` from the database by its id.
    pub async fn delete(db: &DatabaseHandler, id: i32) -> Result<PgQueryResult, Error> {
        sqlx::query!("DELETE FROM articles WHERE id = $1", id)
            .execute(&db.pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Search is confirmed working (I think); no need to run this.
    /// NEVER RUN THIS AGAINST A PRODUCTION DATABASE!
    #[test]
    fn test_search() {
        async fn test() {
            let db = DatabaseHandler::create()
                .await
                .expect("FAILED TO CONNECT TO DATABASE");

            let text = Text::create(
                "Katter och hundar",
                "author",
                "content",
                TextType::Article,
                vec![],
            );

            text.save_to_db(&db).await.expect("SAVING FAILED");

            let results = Text::search(&db, "katt")
                .await
                .expect("ERR WHILE SEARCHING");

            println!("{:?}", results);
        }

        tokio_test::block_on(test())
    }
}

