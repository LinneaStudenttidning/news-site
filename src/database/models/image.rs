use std::{fs, io::Cursor};

use chrono::{DateTime, Local};
use image::{imageops::FilterType::Triangle, load, ImageFormat};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::{database::DatabaseHandler, defaults::DATA_DIR, error::Error};

/// Max width of a small image.
const IMG_S_SIZE: u32 = 600;
/// Max width of a medium image.
const IMG_M_SIZE: u32 = 1200;

/// `Image` represents the metadata of an image.
/// It is stored in the database. The actual image files are stored in:
/// `${DATA_DIR}/images/{s,m,l}`
///
/// The `s`, `m`, and `l` represent different sizes (max width) of the image:
/// * `s` - 600
/// * `m` - 1200
/// * `l` - Original image size
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    pub id: Uuid,
    pub author: String,
    pub description: Option<String>,
    pub created_at: DateTime<Local>,
    pub tags: Vec<String>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            author: "UNKNOWN!".into(),
            description: None,
            created_at: Local::now(),
            tags: Vec::new(),
        }
    }
}

impl Image {
    /// Create a new `Image`; this should be prefered over manually creating a new `Image`.
    pub fn create(author: &str, description: Option<&str>, tags: Vec<String>) -> Self {
        Self {
            author: author.into(),
            description: description.map(Into::into),
            tags: tags.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Saves an instance of `Image` to the database.
    pub async fn save_to_db(&self, db: &DatabaseHandler) -> Result<Self, Error> {
        sqlx::query_file_as!(
            Self,
            "sql/images/insert.sql",
            self.id,
            self.author,
            self.description,
            &self.tags
        )
        .fetch_one(&db.pool)
        .await
        .map_err(Error::from)
    }

    /// Saves image data to a file.
    /// It saves three versions (max width):
    /// * `s` - 600
    /// * `m` - 1200
    /// * `l` - Original image size
    pub fn save_to_file(
        id: Uuid,
        image_data: &[u8],
        image_format: ImageFormat,
    ) -> Result<(), Error> {
        // Load in the image to a `DynamicImage`
        let image_data = load(Cursor::new(image_data), image_format)?;

        // Create different sizes of the image.
        let s_image = image_data.resize_to_fill(
            IMG_S_SIZE,
            IMG_S_SIZE * image_data.height() / image_data.width(),
            Triangle,
        );
        let m_image = image_data.resize_to_fill(
            IMG_M_SIZE,
            IMG_M_SIZE * image_data.height() / image_data.width(),
            Triangle,
        );
        let l_image = image_data;

        // Encode the images as WebP.
        let s_image_as_webp = webp::Encoder::from_image(&s_image)?.encode_simple(true, 100.0)?;
        let m_image_as_webp = webp::Encoder::from_image(&m_image)?.encode_simple(true, 100.0)?;
        let l_image_as_webp = webp::Encoder::from_image(&l_image)?.encode_simple(true, 100.0)?;

        // Save the images.
        fs::write(
            format!("{}/images/s/{}.webp", DATA_DIR, id),
            &*s_image_as_webp,
        )?;
        fs::write(
            format!("{}/images/m/{}.webp", DATA_DIR, id),
            &*m_image_as_webp,
        )?;
        fs::write(
            format!("{}/images/l/{}.webp", DATA_DIR, id),
            &*l_image_as_webp,
        )?;

        Ok(())
    }

    /// Gets ALL `Image`s from the database.
    pub async fn get_by_id(db: &DatabaseHandler, id: Uuid) -> Result<Self, Error> {
        sqlx::query_file_as!(Self, "sql/images/get_by_id.sql", id)
            .fetch_one(&db.pool)
            .await
            .map_err(Error::from)
    }

    /// Gets ALL `Image`s from the database.
    pub async fn get_all(db: &DatabaseHandler) -> Result<Vec<Self>, Error> {
        sqlx::query_file_as!(Self, "sql/images/get_all.sql")
            .fetch_all(&db.pool)
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

    /// Deletes ONE `Image` from the database and file stystem by its id.
    pub async fn delete(db: &DatabaseHandler, id: Uuid) -> Result<PgQueryResult, Error> {
        // Remove all files.
        fs::remove_file(format!("{}/images/s/{}.webp", DATA_DIR, id)).ok();
        fs::remove_file(format!("{}/images/m/{}.webp", DATA_DIR, id)).ok();
        fs::remove_file(format!("{}/images/l/{}.webp", DATA_DIR, id)).ok();

        // Remove from database.
        sqlx::query!("DELETE FROM images WHERE id = $1", id)
            .execute(&db.pool)
            .await
            .map_err(Error::from)
    }
}
