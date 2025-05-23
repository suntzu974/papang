use std::sync::Arc;

use anyhow::{Context, Ok};
use axum::Router;
use sqlx::Postgres;
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;

use crate::{
    auth,
    config::Config,
    database::{DatabaseConnection, PgDatabase},
    expense,
    redis::{CacheConnection, RedisClient},
    state::AppState,
};

pub struct Server<C, D, R>
where
    C: Config,
    D: DatabaseConnection<Postgres>,
    R: CacheConnection,
{
    config: C,
    db: D,
    redis: R,
}

impl<C: Config + std::marker::Sync + 'static> Server<C, PgDatabase, RedisClient> {
    pub async fn new(config: C) -> anyhow::Result<Self> {
        let db = PgDatabase::connect(config.database_url())
            .await
            .context("Failed to create PgDatabase")?;
        let redis = RedisClient::connect(config.redis_url())
            .await
            .context("Failed to create RedisClient")?;
        return Ok(Self { config, db, redis });
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let addr = format!("{}:{}", self.config.host(), self.config.port());
        let listener = TcpListener::bind(&addr)
            .await
            .context("Failed to start tcp connection")?;
        tracing::info!("Listening on http://{addr}");
        axum::serve(listener, self.build_routes())
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .context("Failed to start axum server")?;
        Ok(())
    }

    fn build_routes(&self) -> Router {
        let state = Arc::new(AppState::new(
            self.db.pool(),
            self.redis.client(),
            &self.config,
        ));
        Router::new()
            .merge(auth::handler::router())
            .merge(expense::handler::router())
            .layer(TraceLayer::new_for_http())
            .with_state(state)
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };
        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };
        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    }
}
