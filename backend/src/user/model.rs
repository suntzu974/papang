use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,          // Add this line
    pub verification_token: Option<String>, // Add this line
    pub created_at: Option<NaiveDateTime>,
}
