use serde::{Deserialize, Serialize};
use std::fs;
use crate::structures::stats::Rune; // Import Rune struct

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelicData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BossData {
    pub name: String,
    pub ability: String,
    pub param: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnemyData {
    pub tier_1: Vec<String>,
    pub tier_2: Vec<String>,
    pub tier_3: Vec<String>,
    pub bosses: Vec<BossData>,
}

pub fn load_relics() -> Vec<RelicData> {
    let data = fs::read_to_string("assets/data/relics.json")
        .expect("Unable to read relics.json");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

pub fn load_enemies() -> EnemyData {
    let data = fs::read_to_string("assets/data/enemies.json")
        .expect("Unable to read enemies.json");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

// NEW: Function to load runes
pub fn load_runes() -> Vec<Rune> {
    let data = fs::read_to_string("assets/data/runes.json")
        .expect("Unable to read runes.json");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}