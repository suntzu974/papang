use crate::config::Config;
use std::path::PathBuf;

#[derive(Clone)]
pub struct TestConfig {
    pub database_url: String,
    pub redis_url: String,
    pub host: String,
    pub port: u16,
    pub tls_cert_path: Option<PathBuf>,
    pub tls_key_path: Option<PathBuf>,
}

impl Config for TestConfig {
    fn database_url(&self) -> &str {
        &self.database_url
    }

    fn redis_url(&self) -> &str {
        &self.redis_url
    }

    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn tls_cert_path(&self) -> Option<&str> {
        None
    }

    fn tls_key_path(&self) -> Option<&str> {
        None
    }
}