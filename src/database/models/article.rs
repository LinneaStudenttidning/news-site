use std::fmt::Debug;

use chrono::{NaiveDateTime, Utc};
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};
use slug::slugify;
use sqlx::{
    self,
    postgres::{PgQueryResult, PgRow},
    Error,
};

use crate::database::DatabaseHandler;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, sqlx::Type, FromFormField)]
#[sqlx(type_name = "text_type", rename_all = "lowercase")]
pub enum TextType {
    Article,
    Coverage,
    Opinion,
    Other,
}

impl<'a> FromParam<'a> for TextType {
    type Error = String;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "Article" | "article" => Ok(TextType::Article),
            "Coverage" | "coverage" => Ok(TextType::Coverage),
            "Opinion" | "opinion" => Ok(TextType::Opinion),
            "Other" | "other" => Ok(TextType::Other),
            _ => Err(format!("{} is not a valid TextType", param)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Text {
    pub id: i32,
    pub title: String,
    pub title_slug: String,
    pub author: String,
    pub lead_paragraph: String,
    pub text_body: String,
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
            title_slug: "Missing title slug!".into(),
            author: "NULL".into(),
            lead_paragraph: "Missing lead paragraph!".into(),
            text_body: "Missing text body!".into(),
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
        lead_paragraph: &str,
        text_body: &str,
        text_type: TextType,
        tags: Vec<String>,
    ) -> Self {
        Self {
            title: title.into(),
            author: author.into(),
            lead_paragraph: lead_paragraph.into(),
            text_body: text_body.into(),
            text_type,
            tags,
            ..Default::default()
        }
    }

    /// Saves an instance of `Text` to the database.
    pub async fn save_to_db(&self, db: &DatabaseHandler) -> Result<Text, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/articles/insert.sql",
            self.title,
            slugify(&self.title),
            self.author,
            self.lead_paragraph,
            self.text_body,
            &self.text_type as &TextType,
            &self.tags
        )
        .fetch_one(&db.pool)
        .await
    }

    /// Gets Oup to `n` latest `Text`s from the database.
    pub async fn get_n_latest(db: &DatabaseHandler, n: i64) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_n_latest.sql", n)
            .fetch_all(&db.pool)
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

    /// Gets all the unique tags articles have been tagged with.
    pub async fn get_all_tags(db: &DatabaseHandler) -> Result<Vec<String>, Error> {
        sqlx::query_file_scalar!("sql/articles/get_all_tags.sql")
            .fetch_one(&db.pool)
            .await
            // Result is an `Option<Vec<String>>`, so we have to safely unwrap the `Option`.
            .map(|result| result.unwrap_or_default())
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
                "Lead paragraph",
                "Text body",
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
