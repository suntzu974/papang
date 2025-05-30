use super::{Config, env_provider::EnvProvider, error::ConfigError};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct EnvConfig {
    database_url: Cow<'static, str>,
    host: Cow<'static, str>,
    access_secret: Cow<'static, str>,
    refresh_secret: Cow<'static, str>,
    redis_url: Cow<'static, str>,
    smtp_username: Cow<'static, str>,
    smtp_password: Cow<'static, str>,
    from_email: Cow<'static, str>,
    port: u16,
    tls_cert_path: Option<Cow<'static, str>>,
    tls_key_path: Option<Cow<'static, str>>,
    jwt_secret: Cow<'static, str>,
    jwt_expires_in: i64,
    jwt_maxage: i64,
    email_from: Cow<'static, str>,
    email_password: Cow<'static, str>,
    smtp_server: Cow<'static, str>,
    smtp_port: u16,
    frontend_url: Cow<'static, str>,
}

impl EnvConfig {
    pub fn new<P: EnvProvider>(provider: P) -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: provider.get("DATABASE_URL")?,
            host: provider.get("HOST")?,
            access_secret: provider.get("ACCESS_SECRET")?,
            refresh_secret: provider.get("REFRESH_SECRET")?,
            redis_url: provider.get("REDIS_URL")?,
            smtp_username: provider.get("SMTP_USERNAME")?,
            smtp_password: provider.get("SMTP_PASSWORD")?,
            from_email: provider.get("FROM_EMAIL")?,
            port: provider
                .get("PORT")?
                .parse::<u16>()
                .map_err(ConfigError::InvalidPort)?,
            tls_cert_path: provider.get("TLS_CERT_PATH").ok(),
            tls_key_path: provider.get("TLS_KEY_PATH").ok(),
            jwt_secret: provider.get("JWT_SECRET")?,
            jwt_expires_in: provider
                .get("JWT_EXPIRES_IN")?
                .parse::<i64>()
                .map_err(ConfigError::InvalidJwtExpiresIn)?,
            jwt_maxage: provider
                .get("JWT_MAXAGE")?
                .parse::<i64>()
                .map_err(ConfigError::InvalidJwtMaxage)?,
            email_from: provider.get("EMAIL_FROM")?,
            email_password: provider.get("EMAIL_PASSWORD")?,
            smtp_server: provider.get("SMTP_SERVER")?,
            smtp_port: provider
                .get("SMTP_PORT")?
                .parse::<u16>()
                .map_err(ConfigError::InvalidSmtpPort)?,
            frontend_url: provider.get("FRONTEND_URL")?,

        })
    }
}

impl Config for EnvConfig {
    fn database_url(&self) -> &str {
        &self.database_url
    }

    fn access_secret(&self) -> &str {
        &self.access_secret
    }

    fn refresh_secret(&self) -> &str {
        &self.refresh_secret
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

    fn smtp_username(&self) -> &str {
        &self.smtp_username
    }

    fn smtp_password(&self) -> &str {
        &self.smtp_password
    }

    fn from_email(&self) -> &str {
        &self.from_email
    }

    fn tls_cert_path(&self) -> Option<&str> {
        self.tls_cert_path.as_deref()
    }

    fn tls_key_path(&self) -> Option<&str> {
        self.tls_key_path.as_deref()
    }
    
    fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
    
    fn jwt_expires_in(&self) -> i64 {
        self.jwt_expires_in
    }
    
    fn jwt_maxage(&self) -> i64 {
        self.jwt_maxage
    }
    
    fn email_from(&self) -> &str {
        &self.email_from
    }
    
    fn email_password(&self) -> &str {
        &self.email_password
    }
    
    fn smtp_server(&self) -> &str {
        &self.smtp_server
    }
    
    fn smtp_port(&self) -> u16 {
        self.smtp_port
    }
    
    fn frontend_url(&self) -> &str {
        &self.frontend_url
    }
}
