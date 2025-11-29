use crate::structures::hand::HandRank;

#[derive(Debug)]
pub struct BaseModifiers {
    pub mult: i32,
    pub chips: i32,
    pub total_score: i32,     // The real logic score
    pub display_score: f32,   // The visual counting number
    pub target_score: i32,
    pub deck_count: i32,
    pub hands_remaining: i32,
    pub discards_remaining: i32,
    pub money: i32,
    pub ante: i32,
    pub round: i32,
    pub hand_rank: Option<HandRank>,
}

impl Default for BaseModifiers {
    fn default() -> Self {
        Self {
            mult: 0,
            chips: 0,
            total_score: 0,
            display_score: 0.0,
            target_score: 300,
            deck_count: 52,
            hands_remaining: 4,
            discards_remaining: 3,
            money: 4,
            ante: 1,
            round: 1,
            hand_rank: None,
        }
    }
}