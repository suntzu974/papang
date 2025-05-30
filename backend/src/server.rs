use std::sync::Arc;

use anyhow::{Context, Ok};
use axum::Router;
use sqlx::Postgres;
use tokio::{net::TcpListener, signal};
use tower_http::{
    cors::CorsLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
};
use axum::http::Method;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum_server::tls_rustls::RustlsConfig;
use crate::{
    auth,
    config::Config,
    database::{DatabaseConnection, PgDatabase},
    expense,
    redis::{CacheConnection, RedisClient},
    state::AppState,
};
use std::time::Duration;

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
        
        // Check if HTTPS is enabled
        if let (Some(cert_path), Some(key_path)) = (self.config.tls_cert_path(), self.config.tls_key_path()) {
            tracing::info!("Starting HTTPS server on https://{addr}");
            self.run_https(listener, cert_path, key_path).await
        } else {
            tracing::info!("Starting HTTP server on http://{addr}");
            self.run_http(listener).await
        }
    }

    async fn run_http(&self, listener: TcpListener) -> anyhow::Result<()> {
        axum::serve(listener, self.build_routes())
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .context("Failed to start HTTP server")?;
        Ok(())
    }

    async fn run_https(&self, listener: TcpListener, cert_path: &str, key_path: &str) -> anyhow::Result<()> {
        let config = RustlsConfig::from_pem_file(cert_path, key_path)
            .await
            .context("Failed to load TLS certificates")?;

        // Convert tokio::net::TcpListener to std::net::TcpListener
        let std_listener = listener.into_std()?;

        axum_server::from_tcp_rustls(std_listener, config)
            .serve(self.build_routes().into_make_service())
            .await
            .context("Failed to start HTTPS server")?;
        Ok(())
    }

    fn build_routes(&self) -> Router {
        let state = Arc::new(AppState::new(
            self.db.pool(),
            self.redis.client(),
            &self.config,
        ));
        
        // Configure CORS based on protocol
        let allowed_origin = if self.config.tls_cert_path().is_some() && self.config.tls_key_path().is_some() {
            format!("https://{}:{}", self.config.host(), self.config.port())
        } else {
            "http://localhost:8080".to_string()
        };
        
        let cors = CorsLayer::new()
            .allow_origin(allowed_origin.parse::<axum::http::HeaderValue>().unwrap())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::PUT,
                Method::DELETE,
            ])
            .allow_credentials(true)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
            
        Router::new()
            .merge(auth::handler::router())
            .merge(expense::handler::router())
            // Add performance layers
            .layer(CompressionLayer::new()) // Enable gzip/brotli compression
            .layer(RequestBodyLimitLayer::new(1024 * 1024 * 10)) // 10MB request limit
            .layer(TimeoutLayer::new(Duration::from_secs(30))) // 30s timeout
            .layer(cors)
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
