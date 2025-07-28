use std::sync::Arc;

use super::{bot::Bot, types::Message};
use crate::{services::balance::BalanceService, BotError, Result};

pub async fn handle_welcome_command(bot: Arc<Bot>, chat_id: i64) -> Result<()> {
    let text = "
🔥 Добро пожаловать, товарищ! 🔥

Ты зашел в слоты советского союза!

<b>Что ты тут можешь?</b>
└ Ставить талоны (🎟) на результат однорукого бандита! (🎰️️️️️️). Чтобы сыграть просто отправь смайлик 🎰️️️️️️ отдельным сообщением, чтобы запустился ролл.
└ Проверять свой баланс, выполнив команду /balance

🔥Хорошего отдыха, товрищь! 🔥   
";

    bot.send_message(chat_id, text).await
}

pub async fn handle_balance_command(bot: Arc<Bot>, service: Arc<BalanceService>, msg: &Message) -> Result<()> { 
    let user = match &msg.from {
        Some(user) => user,
        None  => return Err(BotError::Telegram("User not found".to_string())), 
    };
    let balance = match service.handle(user.id).await {
       Ok(Some(balance)) => balance,
       Ok(None) => "Ваш баланс не был найден :с".to_string(),
       Err(e) => return Err(BotError::Telegram(format!("Error while getting balance {:?}", e))), 
    }; 

    bot.send_message(msg.chat.id, &balance).await
}
