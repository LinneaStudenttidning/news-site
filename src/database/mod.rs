use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, error::Error};

pub mod models;

/// A `DatabaseHandler` holds a connection to the database.
pub struct DatabaseHandler {
    /// A connection pool to the database.
    /// This allows multple connections to the database.
    pub pool: PgPool,
}

impl DatabaseHandler {
    /// Creates a new `DatabaseHandler`; this should be prefered over manual initialization.
    pub async fn create() -> Result<Self, Box<dyn Error>> {
        // Load environment variables
        dotenv().ok();

        // Get the database URL
        let db_url = env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(12)
            .connect(&db_url)
            .await?;

        // Run a quick sanity check on the database connection.
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.0, 150);

        Ok(DatabaseHandler { pool })
    }
}

