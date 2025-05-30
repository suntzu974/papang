use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use login::login_handler;
use logout::logout_handler;
use me::{me_handler, update_me_handler};
use refresh::refresh_token_handler;
use register::register_handler;
use verify_email::verify_email_handler;
use resend_verification::resend_verification_handler;
use forgot_password::forgot_password_handler;
use reset_password::reset_password_handler;
use change_password::change_password_handler;

use crate::state::AppState;

pub mod login;
pub mod logout;
pub mod refresh;
pub mod register;
pub mod me;
pub mod verify_email;
pub mod resend_verification;
pub mod forgot_password;
pub mod reset_password;
pub mod change_password;

pub fn router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/verify-email", get(verify_email_handler))
        .route("/auth/me", get(me_handler))
        .route("/auth/me", put(update_me_handler))
        .route("/auth/refresh", post(refresh_token_handler))
        .route("/auth/logout", delete(logout_handler))
        .route("/auth/resend-verification", post(resend_verification_handler))
        .route("/auth/forgot-password", post(forgot_password_handler))
        .route("/auth/reset-password", post(reset_password_handler))
        .route("/auth/change-password", put(change_password_handler));
}
