use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, error::Error};

pub mod models;

pub struct DatabaseHandler {
    pub pool: PgPool,
}

impl DatabaseHandler {
    pub async fn create() -> Result<Self, Box<dyn Error>> {
        // Load environment variables
        dotenv().ok();

        // Get the database URL
        let db_url = env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(12)
            .connect(&db_url)
            .await?;

        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.0, 150);

        Ok(DatabaseHandler { pool })
    }
}
