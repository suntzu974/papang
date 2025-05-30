use std::sync::Arc;

use axum::{
    extract::{Query, State},
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
pub struct VerifyEmailQuery {
    token: String,
}

pub async fn verify_email_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let user = state
        .user_repository
        .verify_email_token(&query.token)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired verification token.".into()))?;

    state.user_repository.mark_email_verified(user.id).await?;

    let response = serde_json::json!({
        "message": "Email verified successfully"
    });

    Ok((StatusCode::OK, Json(response)))
}
