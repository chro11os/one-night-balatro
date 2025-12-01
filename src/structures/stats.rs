use raylib::prelude::*;
use crate::structures::hand::HandRank;
use crate::structures::state::GameState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortMode {
    Rank,
    Suit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BossAbility {
    None,
    SilenceSuit(i32),
    HandSizeMinusOne,
    DoubleTarget,
    PayToDiscard,
}

// NEW: Categorize Runes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuneType {
    Red,   // Aggro/Power
    Blue,  // Utility/Eco
    Green, // Shop/Meta
    Minor, // Stat Shards
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rune {
    pub name: String,
    pub description: String,
    pub id: i32,
    pub rune_type: RuneType,
}

#[derive(Debug)]
pub struct FloatingText {
    pub pos: Vector2,
    pub vel: Vector2,
    pub text: String,
    pub color: Color,
    pub size: i32,
    pub life: f32,
    pub max_life: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vector2,
    pub vel: Vector2,
    pub color: Color,
    pub size: f32,
    pub life: f32,
    pub max_life: f32,
    pub rotation: f32,
    pub rot_speed: f32,
}

#[derive(Debug)]
pub struct BaseModifiers {
    // Balatro Stats
    pub mult: i32,
    pub chips: i32,
    pub total_score: i32,
    pub display_score: f32,
    pub target_score: i32,
    pub deck_count: i32,
    pub hands_remaining: i32,
    pub discards_remaining: i32,
    pub money: i32,
    pub ante: i32,
    pub round: i32,
    pub hand_rank: Option<HandRank>,
    pub hand_size: i32,
    pub joker_slots: i32,

    // Mechanics
    pub shop_price_mult: f32,
    pub ante_scaling: f32,

    // RPG Stats
    pub level: i32,
    pub current_hp: i32,
    pub max_hp: i32,
    pub crit_chance: f32,
    pub crit_mult: f32,

    // Progression
    pub xp: i32,
    pub xp_target: i32,
    pub stat_points: i32,
    pub enemies_defeated: i32,

    pub enemy_name: String,
    pub enemy_damage: i32,
    pub is_crit_active: bool,
    pub active_ability: BossAbility,

    // VFX
    pub floating_texts: Vec<FloatingText>,
    pub particles: Vec<Particle>,

    // NEW: Screen Shake Logic
    pub screen_shake: Vector2,
    pub shake_timer: f32,

    pub score_timer: f32,
    pub score_index: usize,
    pub score_delay: f32,
    pub current_sort: SortMode,

    pub previous_state: GameState,
    pub round_won: bool,

    pub equipped_runes: Vec<Rune>,
    pub available_runes: Vec<Rune>,
    pub max_runes: usize,
}

impl Default for BaseModifiers {
    fn default() -> Self {
        // --- DEFINE RUNES ---
        let mut available_runes = Vec::new();

        // RED RUNES (Power/Aggro)
        available_runes.push(Rune { name: "Reaper".to_string(), description: "Steal 1 HP per enemy. Scales (+1/+2 Boss). Reduced Max HP.".to_string(), id: 1, rune_type: RuneType::Red });
        available_runes.push(Rune { name: "Judgement".to_string(), description: "Balances Chips/Mult. Double Enemy HP.".to_string(), id: 2, rune_type: RuneType::Red });
        available_runes.push(Rune { name: "Paladin".to_string(), description: "+40 Max HP. Reduced Mult/Chips.".to_string(), id: 3, rune_type: RuneType::Red });

        // BLUE RUNES (Utility/Eco)
        available_runes.push(Rune { name: "Midas".to_string(), description: "+25% Gold on Win, -25% Gold on Loss.".to_string(), id: 4, rune_type: RuneType::Blue });
        available_runes.push(Rune { name: "Greed".to_string(), description: "+1 Hand, +1 Discard. -1 Joker Slot.".to_string(), id: 5, rune_type: RuneType::Blue });
        available_runes.push(Rune { name: "Investment".to_string(), description: "Gold gain scales x2 per kill. Less gold early.".to_string(), id: 6, rune_type: RuneType::Blue });

        // GREEN RUNES (Shop/Meta)
        available_runes.push(Rune { name: "Merchant".to_string(), description: "+1 free joker per shop. Shop cost 1.2x more.".to_string(), id: 7, rune_type: RuneType::Green });
        available_runes.push(Rune { name: "Mentalist".to_string(), description: "+1 free Tarot. Tarot cards cost double.".to_string(), id: 8, rune_type: RuneType::Green });
        available_runes.push(Rune { name: "Evolution".to_string(), description: "+1 free Rare Joker per ante. Ante scales faster.".to_string(), id: 9, rune_type: RuneType::Green });

        // MINOR RUNES (Stats)
        available_runes.push(Rune { name: "Force".to_string(), description: "+10 Mult.".to_string(), id: 100, rune_type: RuneType::Minor });
        available_runes.push(Rune { name: "Flow".to_string(), description: "+10 Chips.".to_string(), id: 101, rune_type: RuneType::Minor });
        available_runes.push(Rune { name: "Wealth".to_string(), description: "+3 Gold.".to_string(), id: 102, rune_type: RuneType::Minor });

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
            hand_size: 7,
            joker_slots: 5,

            shop_price_mult: 1.0,
            ante_scaling: 1.5,

            level: 1,
            current_hp: 20,
            max_hp: 20,
            crit_chance: 0.1,
            crit_mult: 1.5,

            xp: 0,
            xp_target: 100,
            stat_points: 0,
            enemies_defeated: 0,

            enemy_name: "Training Dummy".to_string(),
            enemy_damage: 5,
            is_crit_active: false,
            active_ability: BossAbility::None,

            floating_texts: Vec::new(),
            particles: Vec::new(),

            // Screen Shake Init
            screen_shake: Vector2::zero(),
            shake_timer: 0.0,

            score_timer: 0.0,
            score_index: 0,
            score_delay: 0.0,
            current_sort: SortMode::Rank,
            previous_state: GameState::Menu,
            round_won: false,

            equipped_runes: Vec::new(),
            available_runes,
            max_runes: 4,
        }
    }
}