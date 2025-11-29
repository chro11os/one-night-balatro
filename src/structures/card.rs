use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub id: i32,
    pub suit: i32,
    pub value: i32,

    // --- Animation State ---
    pub current_pos: Vector2,
    pub target_pos: Vector2,
    pub velocity: Vector2,

    pub scale: f32,
    pub target_scale: f32,

    pub rotation: f32,
    pub target_rotation: f32,
    pub rot_velocity: f32,

    pub tilt: f32,
    pub is_hovered: bool,
    pub is_selected: bool,

    // NEW: Improved Dragging State
    pub is_dragging: bool, // Actually moving
    pub is_pressed: bool,  // Mouse is down, waiting to see if it becomes a drag
    pub click_pos: Vector2, // Where the mouse was when clicked
}

impl Card {
    pub fn new(id: i32, x: f32, y: f32) -> Self {
        Self {
            id,
            suit: 0,
            value: 0,
            current_pos: Vector2::new(x, 1000.0),
            target_pos: Vector2::new(x, y),
            velocity: Vector2::zero(),
            scale: 0.0,
            target_scale: 1.0,
            rotation: 0.0,
            target_rotation: 0.0,
            rot_velocity: 0.0,
            tilt: 0.0,
            is_hovered: false,
            is_selected: false,

            // Init new fields
            is_dragging: false,
            is_pressed: false,
            click_pos: Vector2::zero(),
        }
    }

    pub fn update_anim(&mut self, dt: f32, total_time: f32) {
        // --- PHYSICS CONFIG ---
        let stiffness = 400.0;
        let damping = 35.0;
        let mass = 1.5;

        // Position Physics
        let force = (self.target_pos - self.current_pos) * stiffness;
        let damping_force = self.velocity * damping;
        let acceleration = (force - damping_force) / mass;
        self.velocity += acceleration * dt;
        self.current_pos += self.velocity * dt;

        // Rotation Physics
        let rot_force = (self.target_rotation - self.rotation) * stiffness;
        let rot_damping = self.rot_velocity * damping;
        let rot_accel = (rot_force - rot_damping) / mass;
        self.rot_velocity += rot_accel * dt;
        self.rotation += self.rot_velocity * dt;

        // Tilt based on movement
        self.tilt = self.velocity.x * 0.05;

        // Scale
        let scale_diff = self.target_scale - self.scale;
        self.scale += scale_diff * 15.0 * dt;

        // Idle Wobble (Only if not interacting)
        if self.is_hovered && !self.is_selected && !self.is_dragging && !self.is_pressed {
            self.target_scale = 1.15;
            self.target_rotation = (total_time * 5.0).sin() * 0.05;
        }
    }
}