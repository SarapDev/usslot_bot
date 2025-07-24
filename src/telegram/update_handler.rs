use std::sync::Arc;

use log::{error, info};

use crate::services::dice::DiceService;
use crate::telegram::types::*;
use crate::telegram::bot::Bot;

pub struct Services {
    dice: Arc<DiceService>,
}

impl Services {
    pub fn new(
        dice: Arc<DiceService>
    ) -> Self {
        Self {
            dice,
        } 
    }
}

/// Struct that handle all types of updates
#[derive(Clone)]
pub struct UpdateHandler {
    services: Arc<Services>, 
    bot: Arc<Bot>,
}

impl UpdateHandler {
    /// Generate object with bot filed
    pub fn new(token: &String, services: Arc<Services>) -> Self {
        Self { 
            services,
            bot: Arc::new(Bot::new(token)),
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

                        let update = update.clone(); // clone for task
                        let handler = self.clone(); 

                        tokio::spawn(async move { 
                            handler.handle_update(&update).await;
                        });
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
        // Handle telegram command (started with /)
        if let Some(text) = &msg.text {
            if text.starts_with("/") {
                self.handle_command(text, msg.chat.id).await; 
                return
            }
        }
        
        // Handle Dice message type
        if let Some(dice) = &msg.dice {
            if let Some(from) = msg.from.as_ref() {
                match self.services.dice.handle(from, dice).await {
                    Ok(Some(text)) => {
                        if let Err(e) = self.bot.send_message(msg.chat.id, &text).await {
                            error!("Error while sending message: {:?}", e);
                        }
                    },
                    Ok(None) => {},
                    Err(e) => error!("Error while handle dice: {:?}", e)
                }

                return
            }
        }            
    }

    /// Function, that execute get some logic to sended command
    pub async fn handle_command(&self, text: &str, chat_id: i64) {
        match text {
            "start" => {
                if let Err(e) = self.bot.send_message(chat_id, "Welcome message").await {
                    error!("Error while sending message: {:?}", e);
                }
            },
            "balance" => {
                if let Err(e) = self.bot.send_message(chat_id, ).await {
                    error!("Error while sending user balance: {:?}", e)
                }
            },
            _ => {},
        }
    }
}
