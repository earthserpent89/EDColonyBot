pub mod sqlite;

pub use sqlite::Database;

use sqlx::{sqlite::{SqlitePool, SqlitePoolOptions}, migrate::MigrateDatabase, Sqlite};
use anyhow::Result;

use crate::models::{construction::ConstructionSite, commodity::Commodity, delivery::Delivery};

const DB_URL: &str = "sqlite:edcolonybot.db";

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        // Create database if it doesn't exist
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            Sqlite::create_database(DB_URL).await?;
        }

        // Create connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(DB_URL)
            .await?;

        // Create tables if they don't exist
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS construction_sites (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                created_by TEXT NOT NULL
            )"
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS commodities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                construction_site_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                quantity_needed INTEGER NOT NULL,
                quantity_delivered INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (construction_site_id) REFERENCES construction_sites (id) ON DELETE CASCADE,
                UNIQUE (construction_site_id, name)
            )"
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS deliveries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                construction_site_id INTEGER NOT NULL,
                commodity_id INTEGER NOT NULL,
                user_id TEXT NOT NULL,
                user_name TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                delivered_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (construction_site_id) REFERENCES construction_sites (id) ON DELETE CASCADE,
                FOREIGN KEY (commodity_id) REFERENCES commodities (id) ON DELETE CASCADE
            )"
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    // Construction site methods
    pub async fn create_construction_site(&self, name: &str, created_by: &str) -> Result<i64> {
        let result = sqlx::query!(
            "INSERT INTO construction_sites (name, created_by) VALUES (?, ?)",
            name,
            created_by
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    // Add more database methods here...
}