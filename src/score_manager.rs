
use crate::structures::stats::{BaseModifiers};
use crate::structures::card::Card;
use crate::structures::relic::{Relic, GameRelic, ScoringContext, RelicEffect};
use crate::poker;
use crate::structures::hand::HandRank;

#[derive(Debug, Default)]
pub struct ScoreResult {
    pub chips: i32,
    pub mult: i32,
}

pub struct ScoreManager;

impl ScoreManager {
    pub fn calculate_hand(played_cards: &[Card], held_cards: &[Card], relics: &[GameRelic], poker_hand: HandRank, stats_snapshot: &BaseModifiers) -> ScoreResult {
        let (current_chips, current_mult) = poker::get_hand_base_score(poker_hand);

        let mut score_result = ScoreResult {
            chips: current_chips,
            mult: current_mult,
        };

        // Create a scoring context for relics to read from
        let mut context = ScoringContext {
            current_chips: score_result.chips,
            current_mult: score_result.mult,
            current_score: 0,
            base_chips: current_chips,
            base_mult: current_mult,
            played_cards,
            held_cards,
            all_cards_in_play: played_cards, // Assuming only played cards are "in play" for most relics
            hand_rank: Some(poker_hand),
            stats_snapshot, // Pass a snapshot of the current stats
        };

        // Step 2: Card Loop (Played Cards) - Trigger OnPlayedCardScored effects
        for card in played_cards {
            for relic in relics {
                match relic.on_played_card_scored(&context, card) {
                    RelicEffect::AddMult(val) => score_result.mult += val,
                    RelicEffect::XMult(val) => score_result.mult = (score_result.mult as f32 * val) as i32,
                    RelicEffect::AddChips(val) => score_result.chips += val,
                    RelicEffect::None => {}
                }
            }
        }

        // Step 3: Held Loop (Held Cards) - Trigger OnHeldCard effects (if any, not explicitly defined yet)
        // For example, if a "Steel Card" relic exists, it could apply an effect here
        // This is largely handled by relics implementing on_held_card_effect (not yet in trait)
        for _card in held_cards {
            for _relic in relics {
                // Example: if a relic has an effect for held cards, apply it here
                // match relic.on_held_card_scored(&context, card) { ... }
            }
        }


        // Step 4: Relic Loop - Apply RelicTrigger::Passive or Global effects (on_hand_scored)
        for relic in relics {
            match relic.on_hand_scored(&context) {
                RelicEffect::AddMult(val) => score_result.mult += val,
                RelicEffect::XMult(val) => score_result.mult = (score_result.mult as f32 * val) as i32,
                RelicEffect::AddChips(val) => score_result.chips += val,
                RelicEffect::None => {}
            }
        }
        
        // Final score calculation
        score_result.chips = score_result.chips.max(0); // Chips can't go below 0
        score_result.mult = score_result.mult.max(0); // Mult can't go below 0

        score_result
    }
}
