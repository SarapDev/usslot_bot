use log::{error, info};
use reqwest::{Client, StatusCode};

use crate::telergam::types::*;
use crate::{handle_dice, Result};
use crate::errors::BotError;

pub struct UpdateHandler {
    token: String,
    client: Client,
    base_url: String,
}

impl UpdateHandler {
    pub fn new(token: String) -> Self {
        info!("Initialize bot");
        info!("{}", token);
        let base_url = format!("https://api.telegram.org/bot{}", token);

        Self { 
            token,
            client: Client::new(), 
            base_url,
        }
    }
    
    async fn get_updates(&self, offset: Option<i64>) -> Result<Vec<Update>> {
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

    pub async fn run(&self) {
        let mut offset = None;
        loop {
            match self.get_updates(offset).await {
                Ok(updates) => {
                    for update in updates {
                        info!("{:?}", update);
                        offset = Some(update.update_id + 1);
                        
                        self.handle_update(&update).await;
                    }  
                },
                Err(e) => error!("Fail to get update {}", e),
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }    
    }

    pub async fn handle_update(&self, update: &Update) { 
        match update.get_type() {
            UpdateType::Message(msg) => self.handle_message(msg).await,
            UpdateType::EditedMessage(_msg) => (),
            UpdateType::CallbackQuery(_callback) => (),
            UpdateType::Unknown => (),
        }
    }

    pub async fn handle_message(&self, msg: &Message) {
        if let Some(dice) = &msg.dice {
            let result = handle_dice(&dice); 

            match self.send_message(msg.chat.id, &result).await {
                Ok(_) => (),
                Err(e) => error!("Error, while sending message. {:?}", e),
            }
        }            
    }
}
