
use crate::structures::stats::{BaseModifiers};
use crate::structures::data_loader::RelicData;
use crate::structures::card::Card;
use crate::structures::hand::HandRank;

#[derive(Clone, Debug)]
pub enum RelicEffect {
    PlusMult(i32),
    XMult(f32),
    PlusChips(i32),
    None, // For passive utility relics
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
    pub effect: RelicEffect, // New field for the relic's effect
}

impl Relic for GameRelic {
    fn id(&self) -> String { self.data.id.clone() }
    fn name(&self) -> String { self.data.name.clone() }

    // These methods will now simply return None as the effect is stored directly
    fn on_hand_scored(&self, _context: &ScoringContext) -> RelicEffect { RelicEffect::None }
    fn on_played_card_scored(&self, _context: &ScoringContext, _card: &Card) -> RelicEffect { RelicEffect::None }
    fn on_hand_end(&self, _context: &ScoringContext) -> RelicEffect { RelicEffect::None }
    fn on_discard(&self, _context: &ScoringContext, _card: &Card) -> RelicEffect { RelicEffect::None }
}
