use log::{error, info};

use crate::telergam::types::*;
use crate::handle_dice;
use crate::telergam::bot::Bot;

/// Struct that handle all types of updates
pub struct UpdateHandler {
    bot: Bot,
}

impl UpdateHandler {
    /// Generate object with bot filed
    pub fn new(token: &String) -> Self {
        Self { 
            bot: Bot::new(token) 
        }
    }

    /// Running main update telegram handler 
    pub async fn run(&self) {
        let mut offset = None;
        loop {
            match self.bot.get_updates(offset).await {
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

    /// Handling update by the type
    pub async fn handle_update(&self, update: &Update) { 
        match update.get_type() {
            UpdateType::Message(msg) => self.handle_message(msg).await,
            UpdateType::EditedMessage(_msg) => (),
            UpdateType::CallbackQuery(_callback) => (),
            UpdateType::Unknown => (),
        }
    }
    
    /// Handle text message update
    pub async fn handle_message(&self, msg: &Message) {
        if let Some(dice) = &msg.dice {
            let result = handle_dice(&dice); 

            match self.bot.send_message(msg.chat.id, &result).await {
                Ok(_) => (),
                Err(e) => error!("Error, while sending message. {:?}", e),
            }
        }            
    }
}
