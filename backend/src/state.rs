use std::sync::Arc;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use ahash::AHasher;
use std::hash::BuildHasherDefault;

use fred::prelude::Client as RedisClient;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use sqlx::PgPool;

use crate::{
    auth::{
        password::PasswordServiceImpl,
        token::{
            repository::refresh_token::RedisRefreshTokenRepository,
            service::{
                access_token::AccessTokenServiceImpl, refresh_token::RefreshTokenServiceImpl,
            },
        },
    },
    config::Config,
    email::EmailService,
    expense::repository::ExpenseRepository,
    user::repository::UserRepositoryImpl,
};

// Global cache for frequently accessed data
static GLOBAL_CACHE: Lazy<DashMap<String, String, BuildHasherDefault<AHasher>>> = 
    Lazy::new(|| DashMap::with_hasher(BuildHasherDefault::default()));

pub struct AppState {
    pub user_repository: UserRepositoryImpl,
    pub access_token_service: AccessTokenServiceImpl,
    pub refresh_token_service: RefreshTokenServiceImpl<RedisRefreshTokenRepository>,
    pub password_service: PasswordServiceImpl,
    pub expense_repository: ExpenseRepository,
    pub email_service: EmailService,
    // Use RwLock for better read performance when writes are infrequent
    pub config_cache: Arc<RwLock<DashMap<String, String, BuildHasherDefault<AHasher>>>>,
    // Connection pools are already optimized
    pub db_pool: sqlx::PgPool,
    pub redis_client: fred::prelude::Client,
}

impl AppState {
    pub fn new<C: Config>(db: Arc<PgPool>, redis_client: Arc<RedisClient>, config: &C) -> Self {
        let config_cache = Arc::new(RwLock::new(DashMap::with_hasher(BuildHasherDefault::default())));
        
        // Pre-populate config cache with frequently accessed values
        {
            let mut cache = config_cache.write();
            cache.insert("jwt_secret".to_string(), config.jwt_secret().to_string());
            cache.insert("frontend_url".to_string(), config.frontend_url().to_string());
        }

        let user_repository = UserRepositoryImpl::new(db.clone());
        let refresh_token_repo = RedisRefreshTokenRepository::new(redis_client.clone());
        let access_token_service = AccessTokenServiceImpl::new(config.access_secret());
        let refresh_token_service =
            RefreshTokenServiceImpl::new(refresh_token_repo, config.refresh_secret());
        let password_service = PasswordServiceImpl::new();
        let expense_repository = ExpenseRepository::new(db.clone());
        let email_service = EmailService::new(
            config.smtp_username().to_string(),
            config.smtp_password().to_string(),
            config.from_email().to_string(),
        );

        Self {
            user_repository,
            access_token_service,
            refresh_token_service,
            password_service,
            expense_repository,
            email_service,
            config_cache,
            db_pool: (*db).clone(),
            redis_client: (*redis_client).clone(),
        }
    }

    pub fn get_cached_config(&self, key: &str) -> Option<String> {
        self.config_cache.read().get(key).map(|v| v.clone())
    }

    pub fn set_cached_config(&self, key: String, value: String) {
        self.config_cache.write().insert(key, value);
    }
}
