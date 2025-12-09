use serde::de::DeserializeOwned;
use std::fs;
use crate::structures::assets::GameAssets;
use crate::structures::relic::{GameRelic, RelicEffect};
use crate::structures::consumable::Consumable;
use crate::structures::heirloom::Heirloom;
use crate::structures::enemy::Enemy;
use crate::structures::stats::Rune; // <--- NEW IMPORT

// Generic JSON Helper
fn load_json<T: DeserializeOwned>(path: &str) -> Vec<T> {
    let content = fs::read_to_string(path).unwrap_or_else(|_| {
        println!("Warning: Could not read {}", path);
        "[]".to_string()
    });

    serde_json::from_str(&content).unwrap_or_else(|e| {
        println!("Error parsing {}: {}", path, e);
        Vec::new()
    })
}

// Relic Data Wrapper
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RelicData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: Option<i32>,
    #[serde(default = "default_rarity")]
    pub rarity: String,
}

fn default_rarity() -> String {
    "Common".to_string()
}

// --- MAIN LOADER ---
pub fn load_all_data(assets: &mut GameAssets) {
    println!("--- LOADING DATA ---");

    // 1. Load Relics
    let relic_list: Vec<RelicData> = load_json("assets/data/relics.json");
    for data in relic_list {
        let game_relic = GameRelic {
            data,
            effect: RelicEffect::None,
        };
        assets.relics_db.insert(game_relic.data.id.clone(), game_relic);
    }
    println!("Loaded {} Relics", assets.relics_db.len());

    // 2. Load Scrolls
    let scrolls: Vec<Consumable> = load_json("assets/data/scrolls.json");
    for item in scrolls {
        assets.consumables_db.insert(item.id.clone(), item);
    }

    // 3. Load Books
    let books: Vec<Consumable> = load_json("assets/data/books.json");
    for item in books {
        assets.consumables_db.insert(item.id.clone(), item);
    }
    println!("Loaded {} Consumables", assets.consumables_db.len());

    // 4. Load Heirlooms
    let heirlooms: Vec<Heirloom> = load_json("assets/data/heirlooms.json");
    for item in heirlooms {
        assets.heirlooms_db.insert(item.id.clone(), item);
    }
    println!("Loaded {} Heirlooms", assets.heirlooms_db.len());

    // 5. Load Enemies
    let enemies: Vec<Enemy> = load_json("assets/data/enemies.json");
    for item in enemies {
        assets.enemies_db.insert(item.id.clone(), item);
    }
    println!("Loaded {} Enemies", assets.enemies_db.len());

    // 6. Load Runes (FIX: This was missing!)
    let runes: Vec<Rune> = load_json("assets/data/runes.json");
    for item in runes {
        assets.runes_db.insert(item.id.clone(), item);
    }
    println!("Loaded {} Runes", assets.runes_db.len());

    println!("--- DATA LOAD COMPLETE ---");
}