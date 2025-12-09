use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Heirloom {
    pub id: String,
    pub name: String,
    pub cost: i32,
    pub effect: HeirloomEffect,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum HeirloomEffect {
    None,
    // Add more heirloom effects here as needed
    ModifyPlayerStats { stat: String, amount: i32 },
    GainMoney { amount: i32 },
}