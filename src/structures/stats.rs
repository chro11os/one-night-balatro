use raylib::prelude::*;
use serde::{Serialize, Deserialize};
use crate::structures::state::GameState;
use crate::structures::hand::HandRank;
use crate::structures::data_loader::{EnemyData, RelicData};

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
    pub current_hp: i32,
    pub max_hp: i32,
    pub money: i32,
    pub hands_remaining: i32,
    pub discards_remaining: i32,
    pub hand_size: i32,
    pub ante: i32,
    pub round: i32,
    pub enemies_defeated: i32,
    pub round_won: bool,
    pub crit_chance: f32,
    pub crit_mult: f32,
    pub chips: i32,
    pub mult: i32,
    pub total_score: i32,
    pub display_score: f32,
    pub target_score: i32,
    pub deck_count: i32,
    pub hand_rank: Option<HandRank>,
    pub enemy_name: String,
    pub enemy_damage: i32,
    pub active_ability: BossAbility,
    pub equipped_runes: Vec<Rune>,
    pub available_runes: Vec<Rune>,
    pub equipped_relics: Vec<RelicData>,
    pub enemy_database: Option<EnemyData>,
    pub relic_database: Vec<RelicData>,
    pub floating_texts: Vec<FloatingText>,
    pub particles: Vec<Particle>,
    pub previous_state: GameState,
    pub current_sort: SortMode,
    pub screen_shake: Vector2,
    pub shake_timer: f32,
    pub is_crit_active: bool,
    pub score_index: usize,
    pub score_timer: f32,
    pub score_delay: f32,
    pub shop_price_mult: f32,
    pub ante_scaling: f32,

    // NEW: Flash timer for enemy damage animation
    pub damage_flash_timer: f32,
}

impl Default for BaseModifiers {
    fn default() -> Self {
        Self {
            level: 1, xp: 0, xp_target: 100, stat_points: 0, current_hp: 100, max_hp: 100, money: 10,
            hands_remaining: 4, discards_remaining: 5, hand_size: 8, ante: 1, round: 1, enemies_defeated: 0, round_won: false,
            crit_chance: 0.10, crit_mult: 1.5, chips: 0, mult: 0, total_score: 0, display_score: 0.0, target_score: 300,
            deck_count: 52, hand_rank: None,
            enemy_name: "Giant Rat".to_string(), enemy_damage: 10, active_ability: BossAbility::None,
            equipped_runes: Vec::new(), available_runes: Vec::new(), equipped_relics: Vec::new(),
            enemy_database: None, relic_database: Vec::new(),
            floating_texts: Vec::new(), particles: Vec::new(),
            previous_state: GameState::Menu, current_sort: SortMode::Rank,
            screen_shake: Vector2::zero(), shake_timer: 0.0,
            is_crit_active: false, score_index: 0, score_timer: 0.0, score_delay: 0.0,
            shop_price_mult: 1.0, ante_scaling: 1.5,

            // Init new field
            damage_flash_timer: 0.0,
        }
    }
}

impl BaseModifiers {
    pub fn update_vfx(&mut self, dt: f32) {
        // Update Damage Flash
        if self.damage_flash_timer > 0.0 {
            self.damage_flash_timer -= dt;
        }

        // Update Floating Text
        self.floating_texts.retain_mut(|ft| {
            ft.life -= dt;
            ft.pos += ft.vel * dt;
            ft.vel.y *= 0.95; // Drag
            ft.life > 0.0
        });

        // Update Particles
        self.particles.retain_mut(|p| {
            p.life -= dt;
            p.pos += p.vel * dt;
            p.rotation += p.rot_speed * dt;
            p.vel.y += 800.0 * dt; // Gravity
            p.life > 0.0
        });
    }

    pub fn spawn_floating_text(&mut self, text: String, pos: Vector2, color: Color) {
        self.floating_texts.push(FloatingText {
            pos,
            vel: Vector2::new(0.0, -100.0), // Shoot up
            text,
            color,
            size: 40,
            life: 1.2,
            max_life: 1.2,
        });
    }

    pub fn spawn_particle_burst(&mut self, pos: Vector2, color: Color) {
        for _ in 0..15 {
            let angle = unsafe { raylib::ffi::GetRandomValue(0, 360) } as f32 * 0.0174533;
            let speed = unsafe { raylib::ffi::GetRandomValue(150, 400) } as f32;
            let vel = Vector2::new(angle.cos() * speed, angle.sin() * speed);
            let size = unsafe { raylib::ffi::GetRandomValue(6, 14) } as f32;

            self.particles.push(Particle {
                pos,
                vel,
                color,
                size,
                life: 0.6,
                max_life: 0.6,
                rotation: 0.0,
                rot_speed: unsafe { raylib::ffi::GetRandomValue(-300, 300) } as f32,
            });
        }
    }
}