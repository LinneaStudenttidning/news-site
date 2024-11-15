use std::fmt::Debug;

use chrono::{DateTime, Local};
use rocket::{http::Status, request::FromParam};
use serde::{Deserialize, Serialize};
use slug::slugify;
use sqlx::{self, postgres::PgQueryResult, types::Json};
use uuid::Uuid;

use crate::{block_editor::Block, database::DatabaseHandler, error::Error};

use super::{creator::Creator, image::Image};

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
    /// Reference to the an `Image` that is used as the thumbnail.
    pub thumbnail_id: Option<Uuid>,
    pub lead_paragraph: String,
    pub text_body: Json<Vec<Block>>,
    pub text_type: TextType,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub tags: Vec<String>,
    pub is_published: bool,
    pub marked_as_done: bool,
    pub creator: Creator,
    pub thumbnail: Option<Image>,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            id: 0,
            title: "Missing title!".into(),
            title_slug: "Missing title slug!".into(),
            author: "NULL".into(),
            thumbnail_id: None,
            lead_paragraph: "Missing lead paragraph!".into(),
            text_body: Json(Vec::new()),
            text_type: TextType::Other,
            created_at: Local::now(),
            updated_at: Local::now(),
            tags: Vec::new(),
            is_published: false,
            marked_as_done: false,
            creator: Creator::create("Missing name", "Missing Display name", "password", false)
                .unwrap(),
            thumbnail: None,
        }
    }
}

impl Text {
    /// Create a new `Text`; this should be preferred over manually creating a new `Text`.
    pub fn create(
        title: &str,
        author: &str,
        lead_paragraph: &str,
        text_body: Vec<Block>,
        text_type: TextType,
        tags: Vec<String>,
    ) -> Self {
        Self {
            title: title.into(),
            author: author.into(),
            lead_paragraph: lead_paragraph.into(),
            text_body: Json(text_body),
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
            self.thumbnail_id,
            self.lead_paragraph,
            serde_json::to_value(self.text_body.clone())?,
            &self.text_type as &TextType,
            &self.tags,
            self.is_published,
            self.marked_as_done,
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Updates ONE text from the data by its `id`.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_by_id(
        db: &DatabaseHandler,
        id: i32,
        title: &str,
        thumbnail_id: Option<Uuid>,
        lead_paragraph: &str,
        text_body: Json<Vec<Block>>,
        text_type: TextType,
        tags: &Vec<String>,
    ) -> Result<Text, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/articles/update.sql",
            title,
            slugify(title),
            thumbnail_id,
            lead_paragraph,
            serde_json::to_value(text_body)?,
            text_type as TextType,
            tags,
            id,
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Gets up to `n` latest `Text`s from the database.
    pub async fn get_n_latest(
        db: &DatabaseHandler,
        n: i64,
        is_published: bool,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_n_latest.sql", is_published, n,)
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets all done, but unpublished articles.
    pub async fn get_all_done_unpublished(db: &DatabaseHandler) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_all_done_unpublished.sql")
            .fetch_all(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets the count of unpublished articles.
    pub async fn get_all_done_unpublished_count(db: &DatabaseHandler) -> Result<i64, Error> {
        sqlx::query_file_scalar!("sql/articles/get_all_done_unpublished-count.sql")
            .fetch_one(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ONE `Text` from the database by its id.
    /// * `must_be_published` if `true`, returns only if published.
    ///     If false, returns article so long it exists.
    pub async fn get_by_id(
        db: &DatabaseHandler,
        id: i32,
        must_be_published: bool,
    ) -> Result<Self, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_id.sql", id, must_be_published)
            .fetch_one(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ALL `Text`s from the database by `author`.
    pub async fn get_by_author(
        db: &DatabaseHandler,
        author: &str,
        is_published: bool,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/articles/get_by_author.sql", author, is_published)
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

    /// Changes the `is_published` field of a text in the database.
    /// * `executor` is the person who wants to publish the text,
    ///     must be a `Publisher`.
    /// * `id` text's id.
    /// * `status` publish status; `true` for published, `false` for not published.
    pub async fn set_publish_status(
        db: &DatabaseHandler,
        executor: &Creator,
        id: i32,
        status: bool,
    ) -> Result<(), Error> {
        if !executor.is_publisher() {
            return Err(Error::create(
                &format!("{}:{}", file!(), line!()),
                "Must be `Publisher` to set publish status!",
                Status::Unauthorized,
            ));
        }

        sqlx::query!(
            "UPDATE articles SET is_published = $1 WHERE id = $2",
            status,
            id
        )
        .execute(&db.pool)
        .await
        .map(|_| ())
        .map_err(Error::from)
    }

    /// Changes the `marked_as_done` field of a text in the database.
    /// * `executor` is the person who wants to publish the text,
    ///     must be a `Publisher`.
    /// * `id` text's id.
    /// * `status` done status; `true` for done, `false` for not done.
    pub async fn set_done_status(
        db: &DatabaseHandler,
        executor: &Creator,
        id: i32,
        status: bool,
    ) -> Result<(), Error> {
        let text = Self::get_by_id(db, id, false).await?;

        if text.author != executor.username {
            return Err(Error::create(
                &format!("{}:{}", file!(), line!()),
                "Only original author can change sdone status!",
                Status::Unauthorized,
            ));
        }

        sqlx::query!(
            "UPDATE articles SET marked_as_done = $1 WHERE id = $2",
            status,
            id
        )
        .execute(&db.pool)
        .await
        .map(|_| ())
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

            let creator = Creator::create("sven.svensson", "Sven Svensson", "123", false)
                .expect("CREATING USER FAILED");

            creator.save_to_db(&db).await.expect("SAVING USER FAILED");

            let text = Text::create(
                "Katter och hundar",
                "sven.svensson",
                "Lead paragraph",
                Vec::new(),
                TextType::Article,
                vec![],
            );

            text.save_to_db(&db).await.expect("SAVING ARTICLE FAILED");

            let results = Text::search(&db, "katt")
                .await
                .expect("ERR WHILE SEARCHING");

            println!("{:?}", results);
        }

        tokio_test::block_on(test())
    }
}
