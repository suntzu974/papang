use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExpenseCategory {
    Groceries,
    Leisure,
    Electronics,
    Utilities,
    Clothing,
    Health,
    Others,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub category: ExpenseCategory,
    pub amount: BigDecimal,
    pub description: Option<String>,
    pub expense_date: NaiveDateTime,
}

impl PartialEq for Expense {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.category == other.category
            && self.amount.to_string() == other.amount.to_string()
            && self.description == other.description
            && self.expense_date == other.expense_date
    }
}

#[derive(Serialize)]
pub struct ChangePasswordForm {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: Option<bool>,
    pub password_reset_token: Option<String>,
    pub created_at: String,
}
