pub mod env_config;
pub mod env_provider;
pub mod error;

pub trait Config {
    fn database_url(&self) -> &str;
    fn access_secret(&self) -> &str;
    fn refresh_secret(&self) -> &str;
    fn redis_url(&self) -> &str;
    fn host(&self) -> &str;
    fn port(&self) -> u16;
    fn smtp_username(&self) -> &str;
    fn smtp_password(&self) -> &str;
    fn from_email(&self) -> &str;
    fn jwt_secret(&self) -> &str;
    fn jwt_expires_in(&self) -> i64;
    fn jwt_maxage(&self) -> i64;
    fn smtp_server(&self) -> &str;
    fn frontend_url(&self) -> &str;
    fn backend_url(&self) -> &str;

    // TLS configuration methods
    fn tls_cert_path(&self) -> Option<&str>;
    fn tls_key_path(&self) -> Option<&str>;
    fn use_https(&self) -> bool {
        self.tls_cert_path().is_some() && self.tls_key_path().is_some()
    }
}
