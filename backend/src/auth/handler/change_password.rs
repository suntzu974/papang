use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use validator::Validate;
use serde_json::json;

use crate::{
    auth::{password::PasswordService, token::claims::Claims},
    error::AppError,
    state::AppState,
    user::repository::UserRepository,
    validation::ValidatedJson,
};

#[derive(Debug, Validate, Deserialize)]
pub struct ChangePasswordPayload {
    current_password: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    new_password: String,
}

pub async fn change_password_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    ValidatedJson(payload): ValidatedJson<ChangePasswordPayload>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    // Get user
    let user = state
        .user_repository
        .find_by_id(claims.sub)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // Verify current password
    if !state
        .password_service
        .verify_password(&payload.current_password, &user.password_hash)
    {
        return Err(AppError::BadRequest("Current password is incorrect".into()));
    }

    // Hash new password
    let new_password_hash = state.password_service.hash_password(&payload.new_password)?;

    // Update password
    state
        .user_repository
        .update_password(user.id, &new_password_hash)
        .await?;

    Ok((StatusCode::OK, Json(json!({
        "message": "Password changed successfully"
    }))))
}
