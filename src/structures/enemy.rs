use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Enemy {
    pub id: String,
    pub name: String,
    pub hp: i32,
    pub ability: EnemyAbility,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum EnemyAbility {
    None,
    // Add more enemy abilities here as needed
    DealDamage { damage: i32 },
    Heal { amount: i32 },
}