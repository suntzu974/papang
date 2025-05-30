use std::sync::Arc;
use uuid::Uuid; // Import Uuid

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use validator::Validate;
use serde_json::json; // Import json macro
use anyhow::anyhow;

use crate::{
    auth::{
        password::PasswordService,
        // Remove token response and services for now, as tokens are not issued on register
    },
    error::AppError,
    state::AppState,
    user::{repository::UserRepository, utils::CreateUserPayload},
    validation::ValidatedJson,
};

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterPayload {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    name: String,

    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
}

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<RegisterPayload>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> { // Return a generic JSON value
    if state
        .user_repository
        .exists_by_email(&payload.email)
        .await?
    {
        return Err(AppError::Conflict("User already exits".into()));
    }

    let password_hash = state.password_service.hash_password(&payload.password)?;
    
    let verification_token = Uuid::new_v4().to_string();

    let user = state
        .user_repository
        .create(CreateUserPayload {
            name: payload.name,
            email: payload.email.clone(), // Clone email for use in email sending
            password_hash,
            verification_token: verification_token.clone(), // Clone token for use in email sending
        })
        .await?;

    // Send verification email
    if let Err(e) = state.email_service
        .send_verification_email(&payload.email, &verification_token)
        .await
    {
        tracing::error!("Failed to send verification email: {}", e);

        return Err(AppError::InternalServerError(
            anyhow!("Failed to send verification email")
        ));
    }

    Ok((StatusCode::CREATED, Json(json!({
        "message": "Registration successful. Please check your email to verify your account."
    }))))
}