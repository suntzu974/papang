use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use validator::Validate;
use serde_json::json;

use crate::{
    auth::password::PasswordService,
    error::AppError,
    state::AppState,
    user::repository::UserRepository,
    validation::ValidatedJson,
};

#[derive(Debug, Validate, Deserialize)]
pub struct ResetPasswordPayload {
    token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    new_password: String,
}

pub async fn reset_password_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<ResetPasswordPayload>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    // Verify reset token and get user
    let user = state
        .user_repository
        .verify_password_reset_token(&payload.token)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired reset token.".into()))?;

    // Hash new password
    let password_hash = state.password_service.hash_password(&payload.new_password)?;

    // Update password and clear reset token
    state
        .user_repository
        .update_password_and_clear_reset_token(user.id, &password_hash)
        .await?;

    Ok((StatusCode::OK, Json(json!({
        "message": "Password has been successfully reset. You can now log in with your new password."
    }))))
}
