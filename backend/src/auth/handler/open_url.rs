use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use validator::Validate;
use serde_json::json;

use crate::{
    error::AppError,
    state::AppState,
    validation::ValidatedJson,
};

#[derive(Debug, Validate, Deserialize)]
pub struct OpenUrlPayload {
    #[validate(url(message = "Invalid URL format"))]
    pub url: String,
    #[serde(default)]
    pub new_tab: bool,
}

#[derive(Debug, Serialize)]
pub struct OpenUrlResponse {
    pub success: bool,
    pub message: String,
    pub url: String,
}

pub async fn open_url_handler(
    State(_state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<OpenUrlPayload>,
) -> Result<(StatusCode, Json<OpenUrlResponse>), AppError> {
    // Validate URL scheme for security
    let url = url::Url::parse(&payload.url)
        .map_err(|_| AppError::BadRequest("Invalid URL format".into()))?;
    
    // Only allow http and https schemes for security
    match url.scheme() {
        "http" | "https" => {},
        _ => return Err(AppError::BadRequest("Only HTTP and HTTPS URLs are allowed".into())),
    }

    // In a real implementation, you might want to:
    // 1. Log the URL access for security/audit purposes
    // 2. Check against a whitelist/blacklist of URLs
    // 3. Rate limit URL opening requests
    // 4. Validate user permissions

    tracing::info!("URL open requested: {} (new_tab: {})", payload.url, payload.new_tab);

    let response = OpenUrlResponse {
        success: true,
        message: "URL validated and ready to open".to_string(),
        url: payload.url,
    };

    Ok((StatusCode::OK, Json(response)))
}
