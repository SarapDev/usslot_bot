use std::sync::Arc;

use crate::{repository::user::UserRepository, telergam::types::{Dice, User}, Result};

const DEFAULT_BET: i64 = 5;

const LOW_WIN: i64 = 3;
const WIN: i64 = 7;
const BIG_WIN: i64 = 15;

pub struct DiceService {
    repository: Arc<UserRepository>,
}

/// Handling dice results
impl DiceService {
    pub fn new(repository: Arc<UserRepository>) -> Self {
        Self {
            repository,
        }
    }
    
    /// Get types of handling dice
    /// Each dice have own buisness logic
    /// In this bolock we get result for each dice and write data to DB
    pub async fn handle(&self, user: &User, dice: &Dice) -> Result<Option<String>> {
        match dice.emoji.as_str() {
            "🎯"  => Ok(None), 
            "🏀"  => Ok(None), 
            "⚽"  => Ok(None), 
            "🎳"  => Ok(None),
            "🎲"  => Ok(None), 
            "🎰"  => self.hadnle_slot(user, dice).await,
            _ => Ok(Some(format!("Товарищ, ты все поломал!\nРазжукивание: {} {}", dice.emoji.as_str(), dice.value))),
         } 
    }
    
    /// Handle slot sticker and update user profile
    /// If user doesn't exist we create it
    pub async fn hadnle_slot(&self, user: &User, dice: &Dice) -> Result<Option<String>> {
        let base = "Твоя ставка 5 🎟, Товарищ!\n";

        // Calculate winnings based on dice value
        let (win_amount, result_text) = match dice.value {
            1 | 22 | 43 => {
                let win = DEFAULT_BET * WIN;
                (win, format!("{}3 в ряд! Партия будет гордится!\n", base))
            }
            16 | 32 | 48 => {
                let win = DEFAULT_BET * LOW_WIN;
                (win, format!("{}Небольшая победа во имя коммунизма!\n", base))
            }
            64 => {
                let win = DEFAULT_BET * BIG_WIN;
                (win, format!("{}Партия высоко ценит твое достижение, товарищ!\n", base))
            }
            _ => {
                // No win - just deduct the bet
                return self.handle_loss(user).await;
            }
        };

        // Atomic operation: ensure user exists and process the bet
        match self.repository.get_or_create(user).await {
            Ok(_) => {
                // Process bet atomically
                match self.repository.process_bet(user.id, DEFAULT_BET, win_amount).await? {
                    Some(_updated_user) => {
                        Ok(Some(format!("{}Ты выиграл: {}🎟", result_text, win_amount)))
                    }
                    None => {
                        // User doesn't have enough balance
                        Ok(Some("У тебя недостаточно средств для ставки, товарищ! 😔".to_string()))
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn handle_loss(&self, user: &User) -> Result<Option<String>> {
        // Just deduct the bet (win_amount = 0)
        match self.repository.get_or_create(user).await {
            Ok(_) => {
                match self.repository.process_bet(user.id, DEFAULT_BET, 0).await? {
                    Some(_) => Ok(None), // Return None for losses as in original code
                    None => Ok(Some("У тебя недостаточно средств для ставки, товарищ! 😔".to_string())),
                }
            },
            Err(e) => Err(e),
        }
    }

    /// Get user balance safely
    pub async fn get_user_balance(&self, telegram_id: i64) -> Result<i64> {
        match self.repository.get_by_id(telegram_id).await? {
            Some(user) => Ok(user.balance),
            None => Ok(0),
        }
    }
}
