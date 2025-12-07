use raylib::prelude::*;
use crate::utils; // Import the utils module

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingType {
    Linear,
    ElasticOut,
    BackOut,
    EaseOutBounce,
    EaseOutBack,
    EaseInOutQuad,
}

impl EasingType {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingType::Linear => t,
            EasingType::ElasticOut => utils::easing::elastic_out(t),
            EasingType::BackOut => utils::easing::back_out(t),
            EasingType::EaseOutBounce => utils::easing::ease_out_bounce(t),
            EasingType::EaseOutBack => utils::easing::ease_out_back(t),
            EasingType::EaseInOutQuad => utils::easing::ease_in_out_quad(t),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MotionState {
    pub start: Vector2,
    pub end: Vector2,
    pub control: Option<Vector2>, // For Bezier curves
    pub timer: f32,           // Current time elapsed
    pub duration: f32,
    pub easing_type: EasingType,
    pub active: bool,
}

impl MotionState {
    pub fn new(start: Vector2, end: Vector2, duration: f32, easing_type: EasingType) -> Self {
        Self {
            start,
            end,
            control: None,
            timer: 0.0,
            duration,
            easing_type,
            active: true,
        }
    }

    pub fn new_bezier(start: Vector2, end: Vector2, control: Vector2, duration: f32, easing_type: EasingType) -> Self {
        Self {
            start,
            end,
            control: Some(control),
            timer: 0.0,
            duration,
            easing_type,
            active: true,
        }
    }

    pub fn update(&mut self, dt: f32) -> Vector2 {
        if !self.active {
            return self.end; // Return final position if not active
        }

        self.timer += dt;
        let t = (self.timer / self.duration).clamp(0.0, 1.0);
        let eased_t = self.easing_type.apply(t);

        let current_pos = if let Some(control_point) = self.control {
            // Quadratic Bezier Curve: P(t) = (1-t)^2 * P0 + 2*(1-t)*t*P1 + t^2*P2
            let p0 = self.start;
            let p1 = control_point;
            let p2 = self.end;
            p0 * (1.0 - eased_t).powi(2) + p1 * 2.0 * (1.0 - eased_t) * eased_t + p2 * eased_t.powi(2)
        } else {
            // Linear interpolation (eased)
            self.start.lerp(self.end, eased_t)
        };

        if self.timer >= self.duration {
            self.active = false;
            return self.end; // Ensure it snaps to the final position
        }

        current_pos
    }

    pub fn reset_if_inactive(&mut self, current_val: Vector2, target_val: Vector2) {
        if !self.active && current_val == target_val {
            self.timer = 0.0;
            self.active = false; // Ensure it stays inactive if already at target
        }
    }
}