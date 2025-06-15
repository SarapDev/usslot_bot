use config::ConfigError;

#[derive(Debug)]
pub enum BotError {
    Config(ConfigError),
    Http(reqwest::Error),
    Database(mongodb::error::Error),
    Telegram(String),
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl std::fmt::Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BotError::Config(e) => write!(f, "Configuration error: {}", e),
            BotError::Http(e) => write!(f, "HTTP error: {}", e),
            BotError::Database(e) => write!(f, "Database error: {}", e),
            BotError::Telegram(e) => write!(f, "Telegram API error: {}", e),
            BotError::Io(e) => write!(f, "IO error: {}", e),
            BotError::Serde(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for BotError {}

// Convert from other error types
impl From<ConfigError> for BotError {
    fn from(err: ConfigError) -> Self {
        BotError::Config(err)
    }
}

impl From<reqwest::Error> for BotError {
    fn from(err: reqwest::Error) -> Self {
        BotError::Http(err)
    }
}

impl From<mongodb::error::Error> for BotError {
    fn from(err: mongodb::error::Error) -> Self {
        BotError::Database(err)
    }
}

impl From<std::io::Error> for BotError {
    fn from(err: std::io::Error) -> Self {
        BotError::Io(err)
    }
}

impl From<serde_json::Error> for BotError {
    fn from(err: serde_json::Error) -> Self {
        BotError::Serde(err)
    }
}
