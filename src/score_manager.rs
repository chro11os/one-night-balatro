use crate::structures::card::Card;
use crate::structures::relic::{GameRelic, RelicEffect}; // Import GameRelic and RelicEffect

pub struct ScoreResult {
    pub chips: i32,
    pub mult: i32,
    pub total: i32, // chips * mult
}

pub fn calculate_score(
    hand: &[Card], 
    relics: &[GameRelic], // Use the concrete GameRelic struct
    base_chips: i32, 
    base_mult: i32
) -> ScoreResult {
    let mut chips = base_chips;
    let mut mult = base_mult;

    // Step A: Add Played Card Chips
    for card in hand {
        chips += card.value; // Add card enhancements here if you have them
    }

    // Step B: Apply Relics (Left-to-Right Order of Operations)
    for relic in relics {
        match relic.effect {
            RelicEffect::PlusMult(m) => mult += m,
            RelicEffect::PlusChips(c) => chips += c,
            RelicEffect::XMult(x) => mult = (mult as f32 * x) as i32,
            RelicEffect::None => {},
        }
    }

    ScoreResult { chips, mult, total: chips * mult }
}