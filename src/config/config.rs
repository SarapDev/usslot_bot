use config::{Config, ConfigError, Environment, File};
use crate::config::cli::Args;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub bot: Bot,
    pub database: Database,
    pub logger: Logger,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Bot {
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Logger {
    pub log_level: String,
}

impl AppConfig {
    pub fn new(args: &Args) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("logger.log_level", "info")?
            .set_default("database.url", "mongodb://localhost:27017")?
            .set_default("database.name", "telegram_bot")?
            // Add config file
            .add_source(File::with_name(&args.config).required(false))
            // Add environment variables (with prefix)
            .add_source(Environment::with_prefix("TELEGRAM_BOT"))
            // Override with CLI args if provided
            .set_override_option("app.log_level", args.log_level.as_deref())?
            .build()?;

        config.try_deserialize()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.bot.token.is_empty() {
            return Err(ConfigError::Message("Bot token cannot be empty".to_string()));
        }

        Ok(())
    }
}
