use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: Option<bool>,          // Add this line
    pub verification_token: Option<String>, 
    pub password_reset_token: Option<String>,
    pub password_reset_expires_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>, // Add this line
    pub updated_at: Option<NaiveDateTime>
}
