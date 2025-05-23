use std::sync::Arc;

use axum::{extract::State, http::StatusCode};

use crate::{
    auth::token::{claims::Claims, service::refresh_token::RefreshTokenService},
    error::AppError,
    state::AppState,
};

pub async fn logout_handler(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, AppError> {
    state.refresh_token_service.delete_token(claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}
