use std::sync::Arc;

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

pub struct AppState {
    pub user_repository: UserRepositoryImpl,
    pub access_token_service: AccessTokenServiceImpl,
    pub refresh_token_service: RefreshTokenServiceImpl<RedisRefreshTokenRepository>,
    pub password_service: PasswordServiceImpl,
    pub expense_repository: ExpenseRepository,
    pub email_service: EmailService,
}

impl AppState {
    pub fn new<C: Config>(db: Arc<PgPool>, redis_client: Arc<RedisClient>, config: &C) -> Self {
        let user_repository = UserRepositoryImpl::new(db.clone());
        let refresh_token_repo = RedisRefreshTokenRepository::new(redis_client);
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
        }
    }
}
