use raylib::prelude::*;
use crate::structures::tween::Tween; // Import the new Tween struct

#[derive(Debug, Clone)]
pub struct Card {
    pub id: i32,
    pub suit: i32,
    pub value: i32,

    pub current_pos: Vector2,
    pub target_pos: Vector2, // Reintroduce target_pos

    pub scale: Vector2,

    pub rotation: f32,

    pub tilt: f32,
    pub is_hovered: bool,
    pub is_selected: bool,

    pub is_dragging: bool,
    pub is_pressed: bool,
    pub click_pos: Vector2,
    pub tween: Option<Tween>, // New tween field
}

impl Card {
    pub fn new(id: i32, x: f32) -> Self {
        Self {
            id, suit: 0, value: 0,
            current_pos: Vector2::new(x, 1000.0),
            target_pos: Vector2::new(x, 1000.0), // Initialize target_pos
            scale: Vector2::new(1.0, 1.0),
            rotation: 0.0,
            tilt: 0.0,
            is_hovered: false, is_selected: false,
            is_dragging: false, is_pressed: false,
            click_pos: Vector2::zero(),
            tween: None, // Initialize tween as None
        }
    }

    pub fn move_to(&mut self, dest: Vector2, duration: f32) {
        self.tween = Some(Tween::new(self.current_pos, dest, duration));
    }
}