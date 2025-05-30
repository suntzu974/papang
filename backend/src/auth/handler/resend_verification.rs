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
pub struct ResendVerificationPayload {
    #[validate(email(message = "Invalid email format"))]
    email: String,
}

pub async fn resend_verification_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<ResendVerificationPayload>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    // Check if user exists
    let user = state
        .user_repository
        .find_by_email(&payload.email)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // Check if email is already verified
    if user.email_verified.unwrap_or(false) {
        return Ok((StatusCode::OK, Json(json!({
            "message": "Email is already verified"
        }))));
    }

    // Generate new verification token
    let verification_token = Uuid::new_v4().to_string();

    // Update user with new verification token
    state
        .user_repository
        .set_verification_token(user.id, &verification_token)
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

    Ok((StatusCode::OK, Json(json!({
        "message": "Verification email sent successfully. Please check your email."
    }))))
}
