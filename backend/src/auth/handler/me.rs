use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::json;

use crate::{
    error::AppError,
    state::AppState,
    user::repository::UserRepository,
     auth::token::claims::Claims, 
};
pub async fn me_handler(
    State(state): State<Arc<AppState>>,
     claims: Claims,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = state
        .user_repository
        .find_by_id(claims.sub)
        .await?
        .ok_or(AppError::NotFound("User not found".into()))?;

    Ok(Json(json!({
        "id": user.id,
        "name": user.name,
        "email": user.email,
        "created_at": user.created_at
    })))
}
