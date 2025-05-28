use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    auth::token::{
        response::RefreshTokenResponse,
        service::{access_token::AccessTokenService, refresh_token::RefreshTokenService},
    },
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
) -> Result<(StatusCode, Json<RefreshTokenResponse>), AppError> {
    let user = state
        .user_repository
        .verify_email_token(&query.token)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired verification token.".into()))?;

    state.user_repository.mark_email_verified(user.id).await?;

    let response = RefreshTokenResponse {
        access_token: state.access_token_service.generate_token(user.id).await?,
        refresh_token: state.refresh_token_service.generate_token(user.id).await?,
    };

    Ok((StatusCode::OK, Json(response)))
}
