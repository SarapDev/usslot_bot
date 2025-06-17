use log::{error, info};
use reqwest::{Client,StatusCode};

use crate::telergam::types::*;
use crate::Result;
use crate::errors::BotError;

/// Struct, that making request to telegram
pub struct Bot {
    client: Client,
    base_url: String,
}

impl Bot {
    pub fn new(token: &String) -> Self {
        info!("Initialize bot");
        let base_url = format!("https://api.telegram.org/bot{}", token);

        Self { 
            client: Client::new(), 
            base_url,
        }
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
            info!("Received {} updates", result.result.as_ref().map(|v| v.len()).unwrap_or(0));
            Ok(result.result.unwrap_or_default())
        } else {
            error!("Telegram API returned error: {:?}", result.description);
            Err(BotError::Telegram(
                result.description.unwrap_or_else(|| "Unknown error".to_string())
            ))
        }
    }

    /// Sending message to chat with text
    pub async fn send_message(&self, chat_id: i64, text: &String) -> Result<()> {
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
