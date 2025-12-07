use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub id: i32,
    pub suit: i32,
    pub value: i32,

    pub current_pos: Vector2,
    pub target_pos: Vector2,

    pub scale: Vector2,
    pub target_scale: Vector2,

    pub rotation: f32,
    pub target_rotation: f32,

    pub tilt: f32,
    pub is_hovered: bool,
    pub is_selected: bool,

    pub is_dragging: bool,
    pub is_pressed: bool,
    pub click_pos: Vector2,
}

impl Card {
    pub fn new(id: i32, x: f32, y: f32) -> Self {
        Self {
            id, suit: 0, value: 0,
            current_pos: Vector2::new(x, 1000.0),
            target_pos: Vector2::new(x, y),
            scale: Vector2::new(1.0, 1.0),
            target_scale: Vector2::new(1.0, 1.0),
            rotation: 0.0, target_rotation: 0.0,
            tilt: 0.0,
            is_hovered: false, is_selected: false,
            is_dragging: false, is_pressed: false,
            click_pos: Vector2::zero(),
        }
    }
}