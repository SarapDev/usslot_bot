use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::services::balance::BalanceService;
use crate::telegram::command::{handle_balance_command, handle_welcome_command};
use crate::Result;

use log::{error, info};

use crate::services::dice::DiceService;
use crate::telegram::types::*;
use crate::telegram::bot::Bot;

pub struct Services {
    dice: Arc<DiceService>,
    balance: Arc<BalanceService>,
}

impl Services {
    pub fn new(
        dice: Arc<DiceService>,
        balance: Arc<BalanceService>,
    ) -> Self {
        Self {
            dice,
            balance,
        }
    }
}

pub type CommandCallback = Arc<dyn Fn(Arc<Bot>, Arc<Services>,i64) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;

/// Struct that handle all types of updates
#[derive(Clone)]
pub struct UpdateHandler {
    services: Arc<Services>, 
    bot: Arc<Bot>,
    commands: HashMap<String, CommandCallback>
}

impl UpdateHandler {
    /// Generate object with bot filed
    pub fn new(token: &String, services: Arc<Services>) -> Self {
        let mut handler = Self { 
            services,
            bot: Arc::new(Bot::new(token)),
            commands: HashMap::new(),
        };

        handler.register("/start", Arc::new(|bot, _services, chat_id| {
            Box::pin(handle_welcome_command(bot, chat_id))
        }));

        handler.register("/balance", Arc::new(|bot, services, chat_id| {
            Box::pin(handle_balance_command(bot, services.clone().balance.clone(), chat_id))
        }));
        
        handler
    }

    pub fn register(&mut self, name: &str, logic: CommandCallback) {
        self.commands.insert(name.to_string(), logic);
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
        if let Some(callback) = self.commands.get(text) {
            if let Err(e) = callback(self.bot.clone(), self.services.clone(), chat_id).await {
                error!("Error while hande {:?} command. Error: {:?}", text, e) 
            }
        } else {
            error!("Unknown command")
        } 
    }
}
