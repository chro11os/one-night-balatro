use raylib::prelude::*;
use crate::utils::easing;
use crate::structures::motion_state::{MotionState, EasingType};

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub id: i32,
    pub suit: i32,
    pub value: i32,

    pub current_pos: Vector2,
    pub target_pos: Vector2, // The target position for pos_motion

    pub scale: Vector2, // Changed to Vector2 for independent X/Y scaling
    pub target_scale: Vector2, // Changed to Vector2

    pub rotation: f32,
    pub target_rotation: f32,

    pub tilt: f32,
    pub is_hovered: bool,
    pub is_selected: bool,

    pub is_dragging: bool,
    pub is_pressed: bool,
    pub click_pos: Vector2,

    // Position Motion State
    pub pos_motion: MotionState,

    // Scale/Rotation Tweening animation state (kept separate for now as MotionState is Vector2-centric)
    pub scale_rot_animation_start_scale: Vector2,
    pub scale_rot_animation_start_rotation: f32,
    pub scale_rot_animation_timer: f32,
    pub scale_rot_animation_duration: f32,
    pub scale_rot_animation_active: bool,

    // Squash and Stretch animation state
    pub squash_stretch_active: bool,
    pub squash_stretch_timer: f32,
    pub squash_stretch_duration: f32,
    pub squash_stretch_peak_y: f32, // Max stretch along Y axis (e.g., 0.1 for 10% stretch)
    pub squash_stretch_frequency: f32, // How fast the oscillation
    pub squash_stretch_damping: f32, // How fast the oscillation dies
}

impl Card {
    pub fn new(id: i32, x: f32, y: f32) -> Self {
        Self {
            id, suit: 0, value: 0,
            current_pos: Vector2::new(x, 1000.0),
            target_pos: Vector2::new(x, y), // Initial target will be hand position
            scale: Vector2::new(0.0, 0.0), // Initialize as Vector2
            target_scale: Vector2::new(1.0, 1.0), // Initialize as Vector2
            rotation: 0.0, target_rotation: 0.0,
            tilt: 0.0,
            is_hovered: false, is_selected: false,
            is_dragging: false, is_pressed: false,
            click_pos: Vector2::zero(),

            pos_motion: MotionState::new(Vector2::new(x, 1000.0), Vector2::new(x, y), 0.3, EasingType::ElasticOut), // Initial deal animation

            scale_rot_animation_start_scale: Vector2::new(0.0, 0.0),
            scale_rot_animation_start_rotation: 0.0,
            scale_rot_animation_timer: 0.0,
            scale_rot_animation_duration: 0.3, // Default duration
            scale_rot_animation_active: true, // Start active for initial card deal

            squash_stretch_active: false,
            squash_stretch_timer: 0.0,
            squash_stretch_duration: 0.2, // Default duration
            squash_stretch_peak_y: 0.0,
            squash_stretch_frequency: 25.0, // Adjust for bounciness
            squash_stretch_damping: 10.0, // Adjust for decay
        }
    }

    pub fn set_target_pos_tweened(&mut self, new_target: Vector2, duration: f32) {
        if self.target_pos != new_target {
            self.target_pos = new_target;
            self.pos_motion = MotionState::new(self.current_pos, new_target, duration, EasingType::ElasticOut);
        }
    }
    
    // For Bezier trajectories
    pub fn set_target_pos_bezier_tweened(&mut self, new_target: Vector2, control_point: Vector2, duration: f32) {
        if self.target_pos != new_target || self.pos_motion.control != Some(control_point) {
            self.target_pos = new_target;
            self.pos_motion = MotionState::new_bezier(self.current_pos, new_target, control_point, duration, EasingType::BackOut);
        }
    }


    pub fn set_target_scale_rotation_tweened(&mut self, new_scale: Vector2, new_rotation: f32, duration: f32) {
        if self.target_scale != new_scale || self.target_rotation != new_rotation {
            self.scale_rot_animation_start_scale = self.scale;
            self.target_scale = new_scale;
            self.scale_rot_animation_start_rotation = self.rotation;
            self.target_rotation = new_rotation;
            self.scale_rot_animation_timer = 0.0;
            self.scale_rot_animation_duration = duration;
            self.scale_rot_animation_active = true;
        }
    }

    pub fn apply_squash_stretch(&mut self, peak_y: f32, duration: f32) {
        self.squash_stretch_active = true;
        self.squash_stretch_timer = 0.0;
        self.squash_stretch_duration = duration;
        self.squash_stretch_peak_y = peak_y;
    }

    pub fn is_moving(&self) -> bool {
        self.pos_motion.active || self.scale_rot_animation_active || self.squash_stretch_active || (self.current_pos - self.target_pos).length() > 1.0
    }

    pub fn update_anim(&mut self, dt: f32, total_time: f32) {
        // Handle dragging separately
        if self.is_dragging {
            let tilt_target = (self.current_pos.x - self.click_pos.x).clamp(-0.4, 0.4); // Simplified tilt based on drag distance
            self.rotation = tilt_target; // Directly set rotation for dragging
            self.scale = self.target_scale; // Keep scale consistent during drag
            self.pos_motion.active = false; // Disable pos animation
            self.scale_rot_animation_active = false; // Reset scale/rot animation
            self.squash_stretch_active = false; // Disable squash/stretch when dragging
            return; // Skip tweening if dragging
        }

        // Apply hover/selection scale/rotation
        if self.is_hovered && !self.is_selected && !self.is_pressed {
            self.set_target_scale_rotation_tweened(Vector2::new(1.25, 1.25), (total_time * 6.0).sin() * 0.05, 0.1);
        } else {
            self.set_target_scale_rotation_tweened(Vector2::new(1.0, 1.0), 0.0, 0.2);
        }

        // Update position using MotionState
        self.current_pos = self.pos_motion.update(dt);
        if !self.pos_motion.active {
            // Ensure final position is set accurately if animation finished
            self.current_pos = self.target_pos;
        }
        
        // Tweening for scale and rotation (existing logic)
        if self.scale_rot_animation_active {
            self.scale_rot_animation_timer += dt;
            let t = (self.scale_rot_animation_timer / self.scale_rot_animation_duration).clamp(0.0, 1.0);

            let eased_t = easing::elastic_out(t); // Using ElasticOut for scale/rot as well

            self.scale.x = self.scale_rot_animation_start_scale.x + (self.target_scale.x - self.scale_rot_animation_start_scale.x) * eased_t;
            self.scale.y = self.scale_rot_animation_start_scale.y + (self.target_scale.y - self.scale_rot_animation_start_scale.y) * eased_t;
            self.rotation = self.scale_rot_animation_start_rotation + (self.target_rotation - self.scale_rot_animation_start_rotation) * eased_t;

            if self.scale_rot_animation_timer >= self.scale_rot_animation_duration {
                self.scale_rot_animation_active = false;
                self.scale = self.target_scale;
                self.rotation = self.target_rotation;
            }
        }

        // Apply Squash and Stretch effect (on top of current scale)
        if self.squash_stretch_active {
            self.squash_stretch_timer += dt;
            let s_t = (self.squash_stretch_timer / self.squash_stretch_duration).clamp(0.0, 1.0);

            if s_t < 1.0 {
                // Damped sine wave for oscillation
                let oscillation = (1.0 - s_t).powf(self.squash_stretch_damping) * (s_t * self.squash_stretch_frequency * std::f32::consts::PI).sin();
                
                let stretch_y_factor = 1.0 + (self.squash_stretch_peak_y * oscillation);
                let squash_x_factor = 1.0 - (self.squash_stretch_peak_y * oscillation * 0.5); // Squash X by half the Y stretch

                self.scale.y *= stretch_y_factor;
                self.scale.x *= squash_x_factor;
            } else {
                self.squash_stretch_active = false;
            }
        }
    }
}