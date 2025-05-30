use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub struct OptimizedPoolConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl Default for OptimizedPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: num_cpus::get() as u32 * 4, // 4x CPU cores
            min_connections: 2,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600), // 10 minutes
            max_lifetime: Duration::from_secs(1800), // 30 minutes
        }
    }
}

pub async fn create_optimized_pool(
    database_url: &str,
    config: OptimizedPoolConfig,
) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.acquire_timeout)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        // Enable prepared statement caching
        .before_acquire(|conn, meta| Box::pin(async move {
            tracing::debug!("Acquiring connection #{}", meta.age.as_secs());
            Ok(conn)
        }))
        .after_release(|conn, meta| Box::pin(async move {
            tracing::debug!("Releasing connection after {}s", meta.age.as_secs());
            Ok(())
        }))
        .connect(database_url)
        .await
}
