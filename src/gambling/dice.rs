use crate::telergam::types::Dice;

pub fn handle_dice(dice: &Dice) -> String {
    match dice.emoji.as_str() {
        "🎯"  => format!("Bullseye! 🎯 \n Score: {}", dice.value),
        "🏀"  => format!("Scored! 🏀 \n Score: {}", dice.value),
        "⚽"  => format!("Goal! ⚽ \n Score: {}", dice.value),
        "🎳"  => format!("Strike! 🎳 \n Score: {}", dice.value),
        "🎲"  => format!("Strike! 🎳 \n Score: {}", dice.value),
        "🎰"  => format!("JACKPOT! 🎰 \n Score: {}", dice.value),
        _ => format!("Ooops, something go wrong \n DEBUG: {} {}", dice.emoji.as_str(), dice.value),
     } 
}
