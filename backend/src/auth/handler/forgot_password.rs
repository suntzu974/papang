use std::sync::Arc;
use uuid::Uuid;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use validator::Validate;
use serde_json::json;
use anyhow::anyhow;

use crate::{
    error::AppError,
    state::AppState,
    user::repository::UserRepository,
    validation::ValidatedJson,
};

#[derive(Debug, Validate, Deserialize)]
pub struct ForgotPasswordPayload {
    #[validate(email(message = "Invalid email format"))]
    email: String,
}

pub async fn forgot_password_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<ForgotPasswordPayload>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    // Check if user exists
    let user = state
        .user_repository
        .find_by_email(&payload.email)
        .await?;

    // Don't reveal if user exists or not for security reasons
    if user.is_none() {
        return Ok((StatusCode::OK, Json(json!({
            "message": "If an account with this email exists, a password reset link has been sent."
        }))));
    }

    let user = user.unwrap();

    // Check if email is verified
    if user.email_verified != Some(true) {
        return Ok((StatusCode::OK, Json(json!({
            "message": "If an account with this email exists, a password reset link has been sent."
        }))));
    }

    // Generate reset token
    let reset_token = Uuid::new_v4().to_string();

    // Update user with reset token
    state
        .user_repository
        .set_password_reset_token(user.id, &reset_token)
        .await?;

    // Send password reset email
    if let Err(e) = state.email_service
        .send_password_reset_email(&payload.email, &reset_token)
        .await
    {
        tracing::error!("Failed to send password reset email: {}", e);

        return Err(AppError::InternalServerError(
            anyhow!("Failed to send password reset email")
        ));
    }

    Ok((StatusCode::OK, Json(json!({
        "message": "If an account with this email exists, a password reset link has been sent."
    }))))
}
