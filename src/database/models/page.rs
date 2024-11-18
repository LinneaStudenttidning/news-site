use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use sqlx::{self, postgres::PgQueryResult, types::Json};

use crate::{block_editor::Block, database::DatabaseHandler, error::Error};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Page {
    pub path: String,
    pub title: String,
    pub text_body: Json<Vec<Block>>,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            path: "Missing path!".into(),
            title: "Missing title!".into(),
            text_body: Json(Vec::new()),
        }
    }
}

impl Page {
    /// Create a new `Text`; this should be preferred over manually creating a new `Text`.
    pub fn create(path: &str, title: &str, text_body: Vec<Block>) -> Self {
        Self {
            path: path.into(),
            title: title.into(),
            text_body: Json(text_body),
        }
    }

    /// Saves an instance of `Page` to the database.
    pub async fn save_to_db(&self, db: &DatabaseHandler) -> Result<Page, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/pages/insert.sql",
            self.path,
            self.title,
            serde_json::to_value(self.text_body.clone())?,
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Updates ONE `Page` from the data by its `path`.
    pub async fn update_by_path(
        db: &DatabaseHandler,
        old_path: &str,
        new_path: &str,
        title: &str,
        text_body: Json<Vec<Block>>,
    ) -> Result<Page, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/pages/update.sql",
            old_path,
            new_path,
            title,
            serde_json::to_value(text_body)?,
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Gets ONE `Page` from the database by its path.
    pub async fn get_by_path(db: &DatabaseHandler, path: &str) -> Result<Self, Error> {
        sqlx::query_file_as!(Self, "sql/pages/get_by_path.sql", path)
            .fetch_one(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ALL `Page`s from the database.
    pub async fn get_all(db: &DatabaseHandler) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/pages/get_all.sql")
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
