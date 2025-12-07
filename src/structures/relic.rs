
use crate::structures::stats::{BaseModifiers};
use crate::structures::data_loader::RelicData;
use crate::structures::card::Card;
use crate::structures::hand::HandRank;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelicEffect {
    AddMult(i32),
    XMult(f32),
    AddChips(i32),
    None,
}

pub struct ScoringContext<'a> {
    pub current_chips: i32,
    pub current_mult: i32,
    pub current_score: i32,
    pub base_chips: i32,
    pub base_mult: i32,
    pub played_cards: &'a [Card],
    pub held_cards: &'a [Card], // Cards still in hand
    pub all_cards_in_play: &'a [Card], // All cards currently on screen/relevant for effects
    pub hand_rank: Option<HandRank>,
    pub stats_snapshot: &'a BaseModifiers, // Immutable snapshot of game state
}

pub trait Relic {
    fn id(&self) -> String;
    fn name(&self) -> String;

    // Hooks for different triggers
    fn on_hand_scored(&self, _context: &ScoringContext) -> RelicEffect { RelicEffect::None }
    fn on_played_card_scored(&self, _context: &ScoringContext, _card: &Card) -> RelicEffect { RelicEffect::None }
    fn on_hand_end(&self, _context: &ScoringContext) -> RelicEffect { RelicEffect::None }
    fn on_discard(&self, _context: &ScoringContext, _card: &Card) -> RelicEffect { RelicEffect::None }
    // Add other hooks as needed, e.g., on_boss_start, on_shop_enter, etc.
}

// A wrapper struct to implement the Relic trait for RelicData
#[derive(Debug, Clone)]
pub struct GameRelic {
    pub data: RelicData,
}

impl Relic for GameRelic {
    fn id(&self) -> String { self.data.id.clone() }
    fn name(&self) -> String { self.data.name.clone() }

    fn on_hand_scored(&self, context: &ScoringContext) -> RelicEffect {
        match self.name().as_str() {
            "Power Glove" => RelicEffect::AddChips(context.played_cards.len() as i32 * 2), // Example: +2 chips per played card
            "Lucky Horseshoe" => {
                if context.hand_rank == Some(HandRank::Flush) {
                    RelicEffect::AddMult(5)
                } else {
                    RelicEffect::None
                }
            },
            "PlusFourMult" => RelicEffect::AddMult(4), // For the unit test
            "TimesTwoMult" => RelicEffect::XMult(2.0), // For the unit test
            _ => RelicEffect::None,
        }
    }

    fn on_played_card_scored(&self, _context: &ScoringContext, card: &Card) -> RelicEffect {
        match self.name().as_str() {
            "Glass Shard" => {
                if card.value >= 10 { // Face cards or 10
                    RelicEffect::XMult(1.5)
                } else {
                    RelicEffect::None
                }
            },
            _ => RelicEffect::None,
        }
    }

    fn on_hand_end(&self, context: &ScoringContext) -> RelicEffect {
        match self.name().as_str() {
            "Ancient Coin" => RelicEffect::AddChips(context.stats_snapshot.round), // +1 chip per round
            _ => RelicEffect::None,
        }
    }
}
