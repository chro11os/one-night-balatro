use raylib::prelude::*;
use crate::consts::*;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub id: i32,
    pub suit: i32,  // 0-3
    pub value: i32, // 2-14

    // --- Animation State ---
    pub current_pos: Vector2, // VISUAL position (draw this)
    pub target_pos: Vector2,  // LOGIC position (aim for this)
    pub scale: f32,
    pub target_scale: f32,
    pub is_hovered: bool,     // Optimization: Calculate once in logic, read in draw
}

impl Card {
    pub fn new(id: i32, x: f32, y: f32) -> Self {
        Self {
            id,
            suit: 0,
            value: 0,
            current_pos: Vector2::new(x, 800.0), // Start off-screen (animation effect)
            target_pos: Vector2::new(x, y),
            scale: 0.0,
            target_scale: 1.0,
            is_hovered: false,
        }
    }

    // Super Optimized Lerp (Linear Interpolation)
    pub fn update_anim(&mut self, dt: f32) {
        // Move current_pos towards target_pos smoothly
        self.current_pos.x += (self.target_pos.x - self.current_pos.x) * ANIM_SPEED * dt;
        self.current_pos.y += (self.target_pos.y - self.current_pos.y) * ANIM_SPEED * dt;

        // Scale smoothing
        self.scale += (self.target_scale - self.scale) * ANIM_SPEED * dt;
    }
}