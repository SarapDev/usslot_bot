use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use log::{error, info};
use reqwest::{Client,StatusCode};

use crate::telegram::types::*;
use crate::Result;
use crate::errors::BotError;

pub type CommandCallback = Arc<dyn Fn(Bot, i64) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;

/// Struct, that making request to telegram
pub struct Bot {
    client: Client,
    base_url: String,
    commands: HashMap<String, CommandCallback>
}

impl Bot {
    pub fn new(token: &String) -> Self {
        info!("Initialize bot");
        let base_url = format!("https://api.telegram.org/bot{}", token);
        let mut bot = Self { 
            client: Client::new(), 
            base_url,
            commands: HashMap::new(),
        };

        bot.register("start", Arc::new(|bot, chat_id| {
            Box::pin(async move {
                bot.send_message(chat_id, "Welcome message").await
            })
        }));

        bot.register("balance", Arc::new(|bot, chat_id| {
            Box::pin(async move {
                bot.send_message(chat_id, "Balance command").await
            })
        }));

        bot
    }

    pub fn register(&mut self, name: &str, logic: CommandCallback) {
        self.commands.insert(name.to_string(), logic);
    }

    /// Getting updates from telegram
    pub async fn get_updates(&self, offset: Option<i64>) -> Result<Vec<Update>> {
        let mut url = format!("{}/getUpdates", self.base_url);

        if let Some(offset) = offset {
            url.push_str(&format!("?offset={}", offset));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            error!("Failed to get udpdate, status {}", response.status()); 
            return Err(BotError::Http(response.error_for_status().unwrap_err()));
        }

       let result: TelegramResponse<Vec<Update>> = response.json().await?;
        
        if result.ok {
            Ok(result.result.unwrap_or_default())
        } else {
            error!("Telegram API returned error: {:?}", result.description);
            Err(BotError::Telegram(
                result.description.unwrap_or_else(|| "Unknown error".to_string())
            ))
        }
    }

    /// Sending message to chat with text
    pub async fn send_message(&self, chat_id: i64, text: &str) -> Result<()> {
        let url = format!("{}/sendMessage", self.base_url);
        let request = SendMessageRequest { chat_id, text, parse_mode: None };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
            
        if response.status().is_success() {
            let result: TelegramResponse<serde_json::Value> = response.json().await?;
            if result.ok {
                info!("Message sent successfully to chat_id: {}", chat_id);
            } else {
                error!("Telegram API returned error: {:?}", result.description);
                return Err(BotError::Telegram(
                    result.description.unwrap_or_else(|| "Unknown error".to_string())
                ));
            }
        } else {
            error!("Failed to send message to chat_id: {}, status: {}", chat_id, response.status());
            if response.status() == StatusCode::TOO_MANY_REQUESTS { 
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

                return Box::pin(self.send_message(chat_id, text)).await;
            }
            return Err(BotError::Http(response.error_for_status().unwrap_err()));
        }
        
        Ok(())
    }

}
