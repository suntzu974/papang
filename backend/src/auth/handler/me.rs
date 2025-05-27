use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::json;
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::AppError,
    state::AppState,
    user::repository::UserRepository,
     auth::token::claims::Claims, 
};
use crate::validation::ValidatedJson;

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateMePayload {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    name: String,
}

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

pub async fn update_me_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    ValidatedJson(payload): ValidatedJson<UpdateMePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let updated_user = state
        .user_repository
        .update_name(claims.sub, &payload.name)
        .await?
        .ok_or(AppError::NotFound("User not found".into()))?;

    Ok(Json(json!({
        "id": updated_user.id,
        "name": updated_user.name,
        "email": updated_user.email,
        "created_at": updated_user.created_at
    })))
}
