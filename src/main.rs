use anyhow::Context;
use dotenv::dotenv;
use papang::{
    config::{env_config::EnvConfig, env_provider::StdEnv},
    server::Server,
};
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().context("Failed to load .env")?;
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .with_span_events(FmtSpan::NONE)
        .init();

    let config = EnvConfig::new(StdEnv).context("Failed to initialize AppConfig")?;

    let server = Server::new(config)
        .await
        .context("Failed to create Server")?;
    server.run().await.context("Failed to run server")?;
    return Ok(());
}
