use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Consumable {
    pub id: String,
    pub name: String,
    pub rarity: String,
    pub cost: i32,
    pub effect: ConsumableEffect, // Enum: LevelUp, ConvertSuit, etc.
    pub target: Option<String>,   // For Books (e.g., "Straight")
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")] 
pub enum ConsumableEffect {
    LevelUp { chips: i32, mult: i32 },
    ModifyCard { enhancement: String },
    CreateCard { card: String },
    None,
}
