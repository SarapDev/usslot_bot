use log::{error, info, debug};
use reqwest::Client;

use crate::telergam::types::Update;
use crate::Result;
use crate::errors::BotError;

use super::types::TelegramResponse;

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
    
    async fn get_updates(&self) -> Result<Vec<Update>> {
        let url = format!("{}/getUpdates", self.base_url);

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

    pub async fn run(&self) {
        loop {
            match self.get_updates().await {
                Ok(updates) => {
                    for update in updates {
                        info!("{:?}", update)
                    }  
                },
                Err(e) => error!("Fail to get update {}", e),
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }    
    }
}
