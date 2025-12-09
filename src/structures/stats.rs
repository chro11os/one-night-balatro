use raylib::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::structures::state::GameState;
use crate::structures::hand::HandRank;
use crate::logic::metrics::GameMetrics;
use rand::{self, Rng};

use crate::structures::relic::GameRelic;
use crate::structures::consumable::Consumable;
use crate::structures::heirloom::Heirloom;
use crate::structures::enemy::Enemy;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuneType {
    Red, Blue, Green, Minor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rune {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rune_type: RuneType,
    pub cost: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BossAbility {
    None,
    SilenceSuit(i32),
    HandSizeMinusOne,
    DoubleTarget,
    PayToDiscard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortMode {
    Rank, Suit,
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
    pub level: i32,
    pub xp: i32,
    pub xp_target: i32,
    pub stat_points: i32,
    pub round: i32,
    pub ante: i32,
    pub enemies_defeated: i32,
    pub round_won: bool,

    pub current_hp: i32,
    pub max_hp: i32,
    pub money: i32,
    pub hands_remaining: i32,
    pub discards_remaining: i32,
    pub hand_size: i32,

    pub chips: i32,
    pub mult: i32,
    pub total_score: i32,
    pub round_score: i32,
    pub display_score: f32,
    pub target_score: i32,
    pub crit_chance: f32,
    pub crit_mult: f32,
    pub is_crit_active: bool,

    pub deck_count: i32,
    pub hand_rank: Option<HandRank>,
    pub current_hand_text: String,

    pub current_enemy: Option<Enemy>,
    pub enemy_name: String,
    pub enemy_damage: i32,
    pub active_ability: BossAbility,

    pub consumables: Vec<Consumable>,
    pub heirlooms: Vec<Heirloom>,
    pub interest_cap: i32,
    pub equipped_relics: Vec<GameRelic>,
    pub current_shop_relics: Vec<GameRelic>,
    pub equipped_runes: Vec<Rune>,
    pub available_runes: Vec<Rune>, // <--- RESTORED THIS

    // Databases
    pub enemy_database: Option<HashMap<String, Enemy>>,
    pub all_relics: HashMap<String, GameRelic>,
    pub all_consumables: HashMap<String, Consumable>,
    pub all_heirlooms: HashMap<String, Heirloom>,

    pub floating_texts: Vec<FloatingText>,
    pub particles: Vec<Particle>,
    pub previous_state: GameState,
    pub current_sort: SortMode,

    pub score_index: usize,
    pub score_timer: f32,
    pub discard_index: usize,
    pub discard_timer: f32,
    pub score_delay: f32,

    pub shop_price_mult: f32,
    pub ante_scaling: f32,
    pub shop_y_offset: f32,

    pub trauma: f32,
    pub shake_offset: Vector2,
    pub shake_rotation: f32,

    pub hands_remaining_text: String,
    pub discards_remaining_text: String,
    pub chips_text: String,
    pub mult_text: String,
    pub hp_text: String,
    pub money_text: String,
    pub level_text: String,
    pub enemy_hp_text: String,
    pub current_round_text: String,
    pub stat_points_text: String,
    pub max_hp_stat_text: String,
    pub crit_chance_stat_text: String,
    pub crit_mult_stat_text: String,

    pub damage_flash_timer: f32,
    pub window_y_offset: f32,
    pub input_consumed: bool,
    pub game_metrics: GameMetrics,
}

impl Default for BaseModifiers {
    fn default() -> Self {
        Self {
            level: 1, xp: 0, xp_target: 100, stat_points: 0,
            round: 1, ante: 1, enemies_defeated: 0, round_won: false,

            current_hp: 100, max_hp: 100,
            money: 4,
            interest_cap: 25,

            hands_remaining: 4, discards_remaining: 5, hand_size: 8,

            chips: 0, mult: 0, total_score: 0, round_score: 0, display_score: 0.0, target_score: 300,
            crit_chance: 0.10, crit_mult: 1.5, is_crit_active: false,

            deck_count: 52, hand_rank: None,
            current_hand_text: String::new(),

            current_enemy: None,
            enemy_name: "Giant Rat".to_string(),
            enemy_damage: 10,
            active_ability: BossAbility::None,

            consumables: Vec::new(),
            heirlooms: Vec::new(),
            equipped_relics: Vec::new(),
            current_shop_relics: Vec::new(),
            equipped_runes: Vec::new(),
            available_runes: Vec::new(), // <--- RESTORED INIT

            enemy_database: None,
            all_relics: HashMap::new(),
            all_consumables: HashMap::new(),
            all_heirlooms: HashMap::new(),

            floating_texts: Vec::new(), particles: Vec::new(),
            previous_state: GameState::Menu, current_sort: SortMode::Rank,

            score_index: 0, score_timer: 0.0, discard_index: 0, discard_timer: 0.0, score_delay: 0.0,
            shop_price_mult: 1.0, ante_scaling: 1.5,
            shop_y_offset: 0.0,

            trauma: 0.0, shake_offset: Vector2::zero(), shake_rotation: 0.0,

            hands_remaining_text: String::new(),
            discards_remaining_text: String::new(),
            chips_text: String::new(),
            mult_text: String::new(),
            hp_text: String::new(),
            money_text: String::new(),
            level_text: String::new(),
            enemy_hp_text: String::new(),
            current_round_text: String::new(),
            stat_points_text: String::new(),
            max_hp_stat_text: String::new(),
            crit_chance_stat_text: String::new(),
            crit_mult_stat_text: String::new(),

            damage_flash_timer: 0.0,
            window_y_offset: 0.0,
            input_consumed: false,
            game_metrics: GameMetrics::new(),
        }
    }
}

impl BaseModifiers {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }

    pub fn update_screen_shake(&mut self, dt: f32) {
        if self.trauma > 0.0 {
            self.trauma = (self.trauma - dt).max(0.0);
            let shake_intensity = self.trauma * self.trauma;
            let shake_x = (rand::thread_rng().gen_range(-1.0..1.0) * 10.0) * shake_intensity;
            let shake_y = (rand::thread_rng().gen_range(-1.0..1.0) * 10.0) * shake_intensity;
            self.shake_offset = Vector2::new(shake_x, shake_y);
            self.shake_rotation = (rand::thread_rng().gen_range(-1.0..1.0) * 5.0) * shake_intensity;
        } else {
            self.shake_offset = Vector2::zero();
            self.shake_rotation = 0.0;
        }
    }

    pub fn update_vfx(&mut self, dt: f32) {
        if self.damage_flash_timer > 0.0 { self.damage_flash_timer -= dt; }
        self.floating_texts.retain_mut(|ft| {
            ft.life -= dt;
            ft.pos += ft.vel * dt;
            ft.vel.y *= 0.95;
            ft.life > 0.0
        });
        self.particles.retain_mut(|p| {
            p.life -= dt;
            p.pos += p.vel * dt;
            p.rotation += p.rot_speed * dt;
            p.vel.y += 800.0 * dt;
            p.life > 0.0
        });
    }

    pub fn update_cached_strings(&mut self) {
        self.hands_remaining_text = format!("Hands: {}", self.hands_remaining);
        self.discards_remaining_text = format!("Disc: {}", self.discards_remaining);
        self.chips_text = format!("{}", self.chips);
        self.mult_text = format!("{}", self.mult);
        self.hp_text = format!("{}/{}", self.current_hp, self.max_hp);
        self.money_text = format!("$ {}", self.money);
        self.level_text = format!("{}", self.level);
        let remaining_hp = (self.target_score - self.display_score as i32).max(0);
        self.enemy_hp_text = format!("{} / {}", remaining_hp, self.target_score);
        self.current_round_text = format!("Round {}", self.round);
        self.stat_points_text = format!("Points Available: {}", self.stat_points);
        self.max_hp_stat_text = format!("{}", self.max_hp);
        self.crit_chance_stat_text = format!("{:.0}%", self.crit_chance * 100.0);
        self.crit_mult_stat_text = format!("{:.1}x", self.crit_mult);
    }

    pub fn decrement_hands(&mut self, amount: i32) -> Result<(), String> {
        if self.hands_remaining >= amount {
            self.hands_remaining -= amount;
            Ok(())
        } else {
            Err("Not enough hands remaining.".to_string())
        }
    }

    pub fn decrement_discards(&mut self, amount: i32) -> Result<(), String> {
        if self.discards_remaining >= amount {
            self.discards_remaining -= amount;
            Ok(())
        } else {
            Err("Not enough discards remaining.".to_string())
        }
    }

    pub fn add_money(&mut self, amount: i32) {
        self.money += amount;
    }
}

pub fn spawn_floating_text(stats: &mut BaseModifiers, text: String, pos: Vector2, color: Color) {
    stats.floating_texts.push(FloatingText {
        pos,
        vel: Vector2::new(0.0, -100.0),
        text,
        color,
        size: 40,
        life: 1.2,
        max_life: 1.2,
    });
}

pub fn spawn_particle_burst(stats: &mut BaseModifiers, pos: Vector2, color: Color) {
    for _ in 0..15 {
        let angle = unsafe { raylib::ffi::GetRandomValue(0, 360) } as f32 * 0.0174533;
        let speed = unsafe { raylib::ffi::GetRandomValue(150, 400) } as f32;
        let vel = Vector2::new(angle.cos() * speed, angle.sin() * speed);
        let size = unsafe { raylib::ffi::GetRandomValue(6, 14) } as f32;
        stats.particles.push(Particle {
            pos, vel, color, size, life: 0.6, max_life: 0.6, rotation: 0.0,
            rot_speed: unsafe { raylib::ffi::GetRandomValue(-300, 300) } as f32,
        });
    }
}