use std::sync::Arc;

use super::bot::Bot;
use crate::{services::balance::BalanceService, Result};

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

pub async fn handle_balance_command(bot: Arc<Bot>, service: Arc<BalanceService>, chat_id: i64) -> Result<()> { 
    let balance = "kek"; 

    bot.send_message(chat_id, &balance).await
}
