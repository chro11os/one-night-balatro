use raylib::prelude::*;
use crate::structures::hand::HandRank;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortMode {
    Rank,
    Suit,
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

    // Enemy Stats
    pub enemy_name: String,
    pub enemy_damage: i32,
    pub is_crit_active: bool,

    // Visual Effects
    pub floating_texts: Vec<FloatingText>,

    // Scoring Sequence Logic
    pub score_timer: f32,
    pub score_index: usize,
    pub score_delay: f32,

    // NEW: Sorting State
    pub current_sort: SortMode,
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
            hand_size: 7,

            level: 1,
            current_hp: 20,
            max_hp: 20,
            crit_chance: 0.1,
            crit_mult: 1.5,

            xp: 0,
            xp_target: 100,
            stat_points: 0,

            enemy_name: "Training Dummy".to_string(),
            enemy_damage: 5,
            is_crit_active: false,

            floating_texts: Vec::new(),

            score_timer: 0.0,
            score_index: 0,
            score_delay: 0.0,

            // Default Sort
            current_sort: SortMode::Rank,
        }
    }
}