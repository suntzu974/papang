use crate::error::AppError;
use crate::state::AppState;
use crate::user::repository::UserRepository;
use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct PasswordResetRequest {
    email: String,
}

pub async fn request_password_reset_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PasswordResetRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let user = state
        .user_repository
        .find_by_email(&payload.email)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found.".into()))?;

    let reset_token = Uuid::new_v4().to_string();

    state
        .user_repository
        .update_verification_token(user.id, &reset_token)
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

    let response = serde_json::json!({
        "message": "Password reset email sent successfully"
    });

    Ok((StatusCode::OK, Json(response)))
}