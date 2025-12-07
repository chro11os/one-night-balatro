
use crate::structures::stats::{BaseModifiers};
use crate::structures::data_loader::RelicData;
use crate::structures::card::Card;
use crate::structures::relic::{Relic, GameRelic, ScoringContext, RelicEffect};
use crate::poker; // Assuming poker functions are public
use crate::structures::hand::HandRank;

pub struct ScoreManager; // Simple struct to hold associated functions

impl ScoreManager {
    pub fn calculate_hand_score(
        stats: &mut BaseModifiers,
        played_cards: &[Card],
        held_cards: &[Card],
        hand_rank: HandRank,
        all_relics: &[RelicData], // Pass all relics for context
    ) {
        // Step A: Determine Base Chips and Base Mult from the Poker Hand
        let (mut current_chips, mut current_mult) = poker::get_hand_base_score(hand_rank);

        // Create a scoring context for relics to read from
        let context = ScoringContext {
            current_chips,
            current_mult,
            current_score: 0, // Will be updated later if needed by a relic
            base_chips: current_chips,
            base_mult: current_mult,
            played_cards,
            held_cards,
            all_cards_in_play: played_cards, // Assuming only played cards are "in play" for most relics
            hand_rank: Some(hand_rank),
            stats_snapshot: stats, // Pass a snapshot of the current stats
        };

        // Step B: Iterate through "Played Cards" (triggering card enhancements and specific card-relics)
        // This would involve applying effects from individual cards (e.g., enhancements)
        // For now, we'll simulate a simple played card effect via relic trait.
        for card in played_cards {
            for relic_data in all_relics {
                let game_relic = GameRelic { data: relic_data.clone() };
                match game_relic.on_played_card_scored(&context, card) {
                    RelicEffect::AddMult(val) => current_mult += val,
                    RelicEffect::XMult(val) => current_mult = (current_mult as f32 * val) as i32,
                    RelicEffect::AddChips(val) => current_chips += val,
                    RelicEffect::None => {}
                }
            }
        }


        // Step C: Iterate through "Held Cards" (for steel cards or specific holding effects)
        // Similar to played cards, relics can react to held cards.
        for _card in held_cards {
            // For example, if a "Steel Card" relic exists, it could apply an effect here
            // This is largely handled by relics implementing on_held_card_effect (not yet in trait)
        }

        // Step D: Iterate through the Relic vector (Left-to-Right) applying their effects to the running total
        for relic_data in all_relics {
            let game_relic = GameRelic { data: relic_data.clone() };
            match game_relic.on_hand_scored(&context) {
                RelicEffect::AddMult(val) => current_mult += val,
                RelicEffect::XMult(val) => current_mult = (current_mult as f32 * val) as i32,
                RelicEffect::AddChips(val) => current_chips += val,
                RelicEffect::None => {}
            }
        }

        // Apply calculated chips and mult to global stats
        stats.chips += current_chips;
        stats.mult += current_mult;

        // Update total score
        stats.total_score += stats.chips * stats.mult;
        stats.round_score += stats.chips * stats.mult;
    }
}
