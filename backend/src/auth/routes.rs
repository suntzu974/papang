use std::sync::Arc;

use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    error::AppError,
    state::AppState,
    user::repository::UserRepository,
};

#[derive(Debug, Deserialize)]
pub struct ResendVerificationQuery {
    email: String,
}

pub async fn resend_verification_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResendVerificationQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let user = state
        .user_repository
        .find_by_email(&payload.email)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found.".into()))?;

    if user.email_verified {
        return Err(AppError::Conflict("Email already verified.".into()));
    }

    let verification_token = Uuid::new_v4().to_string();

    state
        .user_repository
        .update_verification_token(user.id, &verification_token)
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

    let response = serde_json::json!({
        "message": "Verification email resent successfully"
    });

    Ok((StatusCode::OK, Json(response)))
}