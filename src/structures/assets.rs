use raylib::prelude::*;
use std::collections::HashMap;

pub struct GameAssets {
    pub tex_cards: HashMap<String, Texture2D>,
}

impl GameAssets {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut tex_cards = HashMap::new();
        let suits = ["clubs", "diamonds", "hearts", "spades"];
        let ranks = ["02", "03", "04", "05", "06", "07", "08", "09", "10", "J", "Q", "K", "A"];

        for suit in suits.iter() {
            for rank in ranks.iter() {
                let key = format!("card_{}_{}", suit, rank);
                let path = format!("assets/Cards/{}.png", key);
                let texture = rl.load_texture(thread, &path).expect(&format!("Failed to load {}", path));
                tex_cards.insert(key, texture);
            }
        }

        Self {
            tex_cards,
        }
    }
}
