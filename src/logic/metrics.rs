#[derive(Debug, Clone)]
pub struct GameMetrics {
    pub clicks: u32,
    pub hands_played: u32,
    pub discards_used: u32,
}

impl GameMetrics {
    pub fn new() -> Self {
        Self { clicks: 0, hands_played: 0, discards_used: 0 }
    }

    pub fn log_click(&mut self, card_index: usize) {
        self.clicks += 1;
        println!("[DEBUG] CLICKED Card Index: {} | Total Clicks: {}", card_index, self.clicks);
    }

    pub fn log_play(&mut self, score: i32) {
        self.hands_played += 1;
        println!(">>> [ACTION] HAND PLAYED | Score: {} | Total Hands: {}", score, self.hands_played);
    }

    pub fn log_discard(&mut self, count: usize) {
        self.discards_used += 1;
        println!(">>> [ACTION] DISCARDED {} cards | Total Discards: {}", count, self.discards_used);
    }
}