pub mod config;
pub mod telergam;
pub mod errors;
pub mod services;
pub mod database;
pub mod repository; 

pub use config::config::AppConfig;
pub use config::cli::{Args, Commands};
pub use telergam::update_handler::UpdateHandler;
pub use errors::bot_error::BotError;
pub use database::connection::*;

pub type Result<T> = std::result::Result<T, BotError>;
