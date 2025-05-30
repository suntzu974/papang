mod auth;
pub mod config;
mod database;
mod error;
mod expense;
mod redis;
pub mod server;
mod state;
mod user;
mod validation;
pub mod email;

// Use mimalloc as the global allocator for better performance
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
