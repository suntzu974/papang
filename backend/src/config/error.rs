use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingVar(&'static str),


    #[error("Invalid PORT: must be a valid u16 number")]
    InvalidPort(std::num::ParseIntError),

    #[error("Invalid JWT_EXPIRES_IN: must be a valid i64 number")]
    InvalidJwtExpiresIn( std::num::ParseIntError),

    #[error("Invalid JWT_MAXAGE: must be a valid i64 number")]
    InvalidJwtMaxage( std::num::ParseIntError),

    #[error("Invalid SMTP_PORT: must be a valid u16 number")]
    InvalidSmtpPort( std::num::ParseIntError),
}
