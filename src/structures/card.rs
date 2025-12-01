use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub id: i32,
    pub suit: i32,
    pub value: i32,

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

    pub is_dragging: bool,
    pub is_pressed: bool,
    pub click_pos: Vector2,
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
            is_dragging: false,
            is_pressed: false,
            click_pos: Vector2::zero(),
        }
    }

    pub fn update_anim(&mut self, dt: f32, total_time: f32) {
        // TWEAKED: Higher stiffness = Snappier. Higher damping = Less wobble.
        let stiffness = 2200.0;
        let damping = 110.0;
        let mass = 1.0;

        let force = (self.target_pos - self.current_pos) * stiffness;
        let damping_force = self.velocity * damping;
        let acceleration = (force - damping_force) / mass;
        self.velocity += acceleration * dt;
        self.current_pos += self.velocity * dt;

        // SCALE: Linear interpolation for snappy grow/shrink
        // 40.0 speed makes it pop instantly
        let scale_diff = self.target_scale - self.scale;
        self.scale += scale_diff * 40.0 * dt;

        // ROTATION LOGIC
        if self.is_dragging {
            // DYNAMIC TILT: Rotate based on how fast we are dragging horizontally
            // This creates the "Sticky/Air Resistance" feel
            let tilt_target = (self.velocity.x * 0.0015).clamp(-0.4, 0.4);
            self.target_rotation = tilt_target;
        } else if self.is_hovered && !self.is_selected && !self.is_pressed {
            self.target_scale = 1.25; // Bigger hover pop
            self.target_rotation = (total_time * 6.0).sin() * 0.05; // Gentle idle sway
        } else {
            self.target_rotation = 0.0;
        }

        // Apply rotation physics
        let rot_force = (self.target_rotation - self.rotation) * 1200.0; // Stiff rotation
        let rot_damping = self.rot_velocity * 60.0;
        let rot_accel = (rot_force - rot_damping) / mass;
        self.rot_velocity += rot_accel * dt;
        self.rotation += self.rot_velocity * dt;
    }
}