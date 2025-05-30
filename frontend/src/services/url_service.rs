use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::services::api_service::ApiService;

#[derive(Serialize)]
pub struct OpenUrlRequest {
    pub url: String,
    pub new_tab: bool,
}

#[derive(Deserialize)]
pub struct OpenUrlResponse {
    pub success: bool,
    pub message: String,
    pub url: String,
}

pub struct UrlService;

impl UrlService {
    pub async fn validate_and_open_url(
        url: &str, 
        new_tab: bool, 
        token: Option<&str>
    ) -> Result<(), String> {
        let payload = OpenUrlRequest {
            url: url.to_string(),
            new_tab,
        };

        let mut request = ApiService::post("/auth/open-url")
            .header("Content-Type", "application/json");

        if let Some(token) = token {
            request = request.header("Authorization", &format!("Bearer {}", token));
        }

        let response = request
            .json(&payload)
            .map_err(|_| "Failed to serialize request")?
            .send()
            .await
            .map_err(|_| "Network error")?;

        if response.status() == 200 {
            let response_data: OpenUrlResponse = response
                .json()
                .await
                .map_err(|_| "Failed to parse response")?;

            if response_data.success {
                // Open the URL in the browser
                Self::open_browser_url(&response_data.url, new_tab)?;
                Ok(())
            } else {
                Err(response_data.message)
            }
        } else {
            Err("Server error".to_string())
        }
    }

    fn open_browser_url(url: &str, new_tab: bool) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window object")?;
        
        if new_tab {
            window
                .open_with_url_and_target(url, "_blank")
                .map_err(|_| "Failed to open URL in new tab")?;
        } else {
            window
                .location()
                .set_href(url)
                .map_err(|_| "Failed to navigate to URL")?;
        }
        
        Ok(())
    }

    pub fn open_external_link(url: &str) {
        if let Err(e) = Self::open_browser_url(url, true) {
            web_sys::console::error_1(&format!("Failed to open URL: {}", e).into());
        }
    }
}
