use std::fmt::Debug;

use chrono::{NaiveDateTime, Utc};
use rocket::{http::Status, request::FromParam};
use serde::{Deserialize, Serialize};
use slug::slugify;
use sqlx::{
    self,
    postgres::{PgQueryResult, PgRow},
};

use crate::{database::DatabaseHandler, error::Error};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, sqlx::Type, FromFormField)]
#[sqlx(type_name = "text_type", rename_all = "lowercase")]
pub enum TextType {
    Article,
    Coverage,
    Opinion,
    Other,
}

impl<'a> FromParam<'a> for TextType {
    type Error = crate::error::Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "Article" | "article" => Ok(TextType::Article),
            "Coverage" | "coverage" => Ok(TextType::Coverage),
            "Opinion" | "opinion" => Ok(TextType::Opinion),
            "Other" | "other" => Ok(TextType::Other),
            _ => Err(Self::Error::create(
                "FromParam for Textformat",
                &format!("{} is not a valid TextType", param),
                Status::BadRequest,
            )),
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
    pub is_published: bool,
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
            is_published: false,
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
        is_published: bool,
    ) -> Self {
        Self {
            title: title.into(),
            author: author.into(),
            lead_paragraph: lead_paragraph.into(),
            text_body: text_body.into(),
            text_type,
            tags,
            is_published,
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
            &self.tags,
            self.is_published
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Updates ONE text from the data by its `id`.
    pub async fn update_by_id(
        db: &DatabaseHandler,
        id: i32,
        title: &str,
        lead_paragraph: &str,
        text_body: &str,
        tags: &Vec<String>,
        is_published: bool,
    ) -> Result<Text, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/articles/update.sql",
            title,
            slugify(title),
            lead_paragraph,
            text_body,
            tags,
            is_published,
            id,
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Gets Oup to `n` latest `Text`s from the database.
    /// The `is_published` defaults to `true` if `None`.
    pub async fn get_n_latest(
        db: &DatabaseHandler,
        n: i64,
        is_published: Option<bool>,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/articles/get_n_latest.sql",
            is_published.unwrap_or(true),
            n,
        )
        .fetch_all(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Gets ONE `Text` from the database by its id.
    /// `check_if_published` defaults to `true` if `None`
    pub async fn get_by_id(
        db: &DatabaseHandler,
        id: i32,
        check_if_published: Option<bool>,
    ) -> Result<Self, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/articles/get_by_id.sql",
            id,
            check_if_published.unwrap_or(true)
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Gets ALL `Text`s from the database by `author`.
    pub async fn get_by_author(db: &DatabaseHandler, author: &str) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_author.sql", author)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ALL `Text`s from the database by `type`.
    pub async fn get_by_type(
        db: &DatabaseHandler,
        text_type: TextType,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_type.sql", text_type as TextType)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ALL `Text`s from the database with `tag`.
    pub async fn get_by_tag(db: &DatabaseHandler, tag: &str) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_tag.sql", tag)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ALL `Text`s from the database with any of `tags`.
    pub async fn get_by_any_of_tags(
        db: &DatabaseHandler,
        tags: &[String],
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_any_of_tags.sql", tags)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ALL `Text`s from the database matching the search query.
    pub async fn search(db: &DatabaseHandler, query: &str) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/search.sql", query)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets all the unique tags articles have been tagged with.
    /// The `limit` defaults to `10` if `None`.
    pub async fn get_all_tags(
        db: &DatabaseHandler,
        limit: Option<i64>,
    ) -> Result<Vec<String>, Error> {
        sqlx::query_file_scalar!("sql/articles/get_all_tags.sql", limit.unwrap_or(10))
            .fetch_one(&db.pool)
            .await
            // Result is an `Option<Vec<String>>`, so we have to safely unwrap the `Option`.
            .map(|result| result.unwrap_or_default())
            .map_err(Error::from)
    }

    /// Publishes a `Text`.
    pub async fn publish(db: &DatabaseHandler, id: i32) -> Result<Vec<PgRow>, Error> {
        sqlx::query!("UPDATE articles SET is_published = true WHERE id = $1", id)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Unpublishes a `Text`.
    pub async fn unpublish(db: &DatabaseHandler, id: i32) -> Result<Vec<PgRow>, Error> {
        sqlx::query!("UPDATE articles SET is_published = false WHERE id = $1", id)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Deletes ONE `Text` from the database by its id.
    pub async fn delete(db: &DatabaseHandler, id: i32) -> Result<PgQueryResult, Error> {
        sqlx::query!("DELETE FROM articles WHERE id = $1", id)
            .execute(&db.pool)
            .await
            .map_err(Error::from)
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
                true, // is_published
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
