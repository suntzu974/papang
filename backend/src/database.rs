use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{Database, Pool, Postgres, postgres::PgPoolOptions};

#[async_trait]
pub trait DatabaseConnection<DB: Database> {
    async fn connect(database_url: &str) -> Result<Self>
    where
        Self: Sized;

    fn pool(&self) -> Arc<Pool<DB>>;
}

pub struct PgDatabase(Arc<Pool<Postgres>>);

#[async_trait]
impl DatabaseConnection<Postgres> for PgDatabase {
    async fn connect(database_url: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await
            .context("can't connect to database")?;
        tracing::info!("Successfully connected to PostgreSQL");

        let migrator = sqlx::migrate!();
        migrator
            .run(&pool)
            .await
            .context("Failed to run migrations")?;

        tracing::info!("Database migrations have been applied (if any were pending).");

        Ok(Self(Arc::new(pool)))
    }

    fn pool(&self) -> Arc<Pool<Postgres>> {
        self.0.clone()
    }
}

impl Drop for PgDatabase {
    fn drop(&mut self) {
        if self.0.is_closed() {
            tracing::info!("PostgreSQL connection was already closed.");
        } else {
            tracing::info!("Closing PostgreSQL connection.");
        }
    }
}
