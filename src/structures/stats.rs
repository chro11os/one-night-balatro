#[derive(Debug)]
pub struct BaseModifiers {
    pub mult: i32,
    pub chips: i32,
    pub total_score: i32,
    pub deck_count: i32,
    pub hands_remaining: i32,
    pub discards_remaining: i32,
}

impl Default for BaseModifiers {
    fn default() -> Self {
        Self {
            mult: 1,
            chips: 0,
            total_score: 0,
            deck_count: 52,
            hands_remaining: 4,
            discards_remaining: 3,
        }
    }
}