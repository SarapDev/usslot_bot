use crate::telergam::types::Dice;

const DEFAULT_BET: f32 = 5.0;

const LOW_WIN: f32 = 1.5;
const WIN: f32 = 3.0;
const BIG_WIN: f32 = 7.0;

pub fn handle_dice(dice: &Dice) -> String {
    match dice.emoji.as_str() {
        "🎯"  => format!("Bullseye! 🎯 \n Score: {}", dice.value),
        "🏀"  => format!("Scored! 🏀 \n Score: {}", dice.value),
        "⚽"  => format!("Goal! ⚽ \n Score: {}", dice.value),
        "🎳"  => format!("Strike! 🎳 \n Score: {}", dice.value),
        "🎲"  => format!("Dice! 🎲 \n Score: {}", dice.value),
        "🎰"  => {
            let base = "Твоя ставка 5🌸!\n";

            match dice.value {
                1 | 22 | 43 => format!("{}3 в ряд!\nТы выиграл: {}🌸", base, DEFAULT_BET * WIN),
                16 | 32 | 48 => format!("{}2 топора лучше, чем ничего\nТы выиграл: {}🌸", base, DEFAULT_BET * LOW_WIN),
                64 => format!("{}Ого, так этож 3 топора!\nТы выиграл: {}🌸", base, DEFAULT_BET * BIG_WIN),
                _ => format!("{}Сори, сегодня я отбираю твои цветочки", base)
            }
        }
        _ => format!("Ooops, something go wrong \n DEBUG: {} {}", dice.emoji.as_str(), dice.value),
     } 
}
