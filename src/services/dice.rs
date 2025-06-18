use crate::telergam::types::Dice;

const DEFAULT_BET: i32 = 5;

const LOW_WIN: i32 = 3;
const WIN: i32 = 7;
const BIG_WIN: i32 = 15;

pub struct DiceService {
}

/// Handling dice results
impl DiceService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(&self, dice: &Dice) -> Option<String> {
        match dice.emoji.as_str() {
            "🎯"  => None, 
            "🏀"  => None, 
            "⚽"  => None, 
            "🎳"  => None,
            "🎲"  => None, 
            "🎰"  => {
                let base = "Твоя ставка 5 🎟, Товарищ!\n";

                match dice.value {
                    1 | 22 | 43 => Some(format!(
                                "{}3 в ряд! Партия будет гордится!\nТы выиграл: {}🎟", 
                                base, 
                                DEFAULT_BET * WIN
                            )),
                    16 | 32 | 48 => Some(format!(
                                "{}Небольшая победа во имя коммунизма!\nТы выиграл: {}🎟",
                                base, 
                                DEFAULT_BET * LOW_WIN,
                            )),
                    64 => Some(format!(
                            "{}Партия высоко ценит твое достижение, товарищ!\nТы выиграл: {}🎟",
                            base,
                            DEFAULT_BET * BIG_WIN,
                            )),
                    _ => None 
                }
            }
            _ => Some(format!("Товарищ, ты все поломал!\nРазжукивание: {} {}", dice.emoji.as_str(), dice.value)),
         } 
    }
}
