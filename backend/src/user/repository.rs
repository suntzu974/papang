use std::sync::Arc;
use uuid::Uuid; // Import Uuid

use super::{model::User, utils::CreateUserPayload};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, payload: CreateUserPayload) -> Result<User>;
    async fn exists_by_email(&self, email: &str) -> Result<bool>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>>;
    async fn update_name(&self, id: i32, name: &str) -> Result<Option<User>>;
    // Add these methods for email verification
    async fn set_verification_token(&self, user_id: i32, token: &str) -> Result<()>;
    async fn verify_email_token(&self, token: &str) -> Result<Option<User>>;
    async fn mark_email_verified(&self, user_id: i32) -> Result<()>;

    // Password reset methods
    async fn set_password_reset_token(&self, user_id: i32, reset_token: &str) -> Result<()>;
    async fn verify_password_reset_token(&self, reset_token: &str) -> Result<Option<User>>;
    async fn update_password_and_clear_reset_token(&self, user_id: i32, password_hash: &str) -> Result<()>;

    // New method to update password without affecting other fields
    async fn update_password(&self, user_id: i32, password_hash: &str) -> Result<()>;

}

pub struct UserRepositoryImpl {
    pool: Arc<Pool<Postgres>>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, payload: CreateUserPayload) -> Result<User> {
        // The verification_token is now passed in the payload
        sqlx::query_as!(
            User,
            "INSERT INTO users (name, email, password_hash,email_verified,verification_token,password_reset_token,password_reset_expires_at) VALUES ($1, $2, $3, FALSE, NULL,NULL,NULL) RETURNING *;",
            payload.name,
            payload.email,
            payload.password_hash
        )
        .fetch_one(&*self.pool)
        .await
        .context("Failed to create user")
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        sqlx::query_as!(User, "SELECT id, name, email, password_hash, email_verified, verification_token,password_reset_token,password_reset_expires_at, created_at,updated_at FROM users WHERE email = $1", email)
            .fetch_optional(&*self.pool)
            .await
            .with_context(|| format!("Failed to find user by email: {}", email))
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool> {
        let exists =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email)
                .fetch_one(&*self.pool)
                .await
                .context("Failed to check if user exists")?;

        Ok(exists.unwrap_or(false))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        sqlx::query_as!(User, "SELECT id, name, email, password_hash, email_verified, verification_token, password_reset_token,password_reset_expires_at,created_at,updated_at FROM users WHERE id = $1", id)
            .fetch_optional(&*self.pool)
            .await
            .with_context(|| format!("Failed to find user by id: {}", id))
    }

    async fn update_name(&self, id: i32, name: &str) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            "UPDATE users SET name = $1 WHERE id = $2 RETURNING *",
            name,
            id
        )
        .fetch_optional(&*self.pool)
        .await
        .with_context(|| format!("Failed to update user name for id: {}", id))
    }

    async fn set_verification_token(&self, user_id: i32, token: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET verification_token = $1 WHERE id = $2",
            token,
            user_id
        )
        .execute(&*self.pool)
        .await
        .map(|_| ())
        .context("Failed to set verification token")
    }

    async fn verify_email_token(&self, token: &str) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            "SELECT id, name, email, password_hash, email_verified, verification_token, password_reset_token,password_reset_expires_at,created_at,updated_at FROM users WHERE verification_token = $1 AND email_verified = FALSE",
            token
        )
        .fetch_optional(&*self.pool)
        .await
        .context("Failed to find user by verification token")
    }

    async fn mark_email_verified(&self, user_id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET email_verified = TRUE, verification_token = NULL WHERE id = $1",
            user_id
        )
        .execute(&*self.pool)
        .await
        .map(|_| ())
        .context("Failed to mark email as verified")
    }

    async fn set_password_reset_token(&self, user_id: i32, reset_token: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET password_reset_token = $1, password_reset_expires_at = NOW() + INTERVAL '1 hour', updated_at = NOW() WHERE id = $2",
            reset_token,
            user_id
        )
        .execute(&*self.pool)
        .await
        .map(|_| ())
        .context("Failed to set password reset token")
    }

    async fn verify_password_reset_token(&self, reset_token: &str) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            "SELECT id, name, email, password_hash, email_verified, verification_token, password_reset_token,password_reset_expires_at,created_at,updated_at FROM users WHERE password_reset_token = $1 AND password_reset_expires_at > NOW()",
            reset_token
        )
        .fetch_optional(&*self.pool)
        .await
        .context("Failed to find user by password reset token")
    }

    async fn update_password_and_clear_reset_token(&self, user_id: i32, password_hash: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET password_hash = $1, password_reset_token = NULL, password_reset_expires_at = NULL, updated_at = NOW() WHERE id = $2",
            password_hash,
            user_id
        )
        .execute(&*self.pool)
        .await
        .map(|_| ())
        .context("Failed to update password and clear reset token")
    }

    async fn update_password(
        &self,
        user_id: i32,
        password_hash: &str,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
            password_hash,
            user_id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
