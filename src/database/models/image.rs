use std::{fs, io::Cursor};

use chrono::{DateTime, Local};
use image::{imageops::FilterType::Triangle, load, ImageFormat};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{database::DatabaseHandler, defaults::DATA_DIR, error::Error};

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

        const S_SIZE: u32 = 600;
        const M_SIZE: u32 = 1200;

        // Create different sizes of the image.
        let s_image = image_data.resize_to_fill(
            S_SIZE,
            S_SIZE * image_data.height() / image_data.width(),
            Triangle,
        );
        let m_image = image_data.resize_to_fill(
            M_SIZE,
            M_SIZE * image_data.height() / image_data.width(),
            Triangle,
        );
        let l_image = image_data;

        // Save images
        let s_image_as_webp = webp::Encoder::from_image(&s_image)?.encode_simple(true, 100.0)?;
        let m_image_as_webp = webp::Encoder::from_image(&m_image)?.encode_simple(true, 100.0)?;
        let l_image_as_webp = webp::Encoder::from_image(&l_image)?.encode_simple(true, 100.0)?;

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
}
