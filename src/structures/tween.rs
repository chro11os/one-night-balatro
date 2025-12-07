use raylib::prelude::*;
use crate::utils::easing;

#[derive(Debug, Clone)]
pub struct Tween {
    start: Vector2,
    pub end: Vector2,
    control: Vector2,
    elapsed: f32,
    duration: f32,
}

impl Tween {
    pub fn new(start: Vector2, end: Vector2, duration: f32) -> Self {
        let control = Vector2::new(
            (start.x + end.x) / 2.0,
            (start.y + end.y) / 2.0 - 150.0,
        );
        Self {
            start,
            end,
            control,
            elapsed: 0.0,
            duration,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        if self.elapsed > self.duration {
            self.elapsed = self.duration;
        }
    }

    pub fn solve(&self) -> Vector2 {
        let t = (self.elapsed / self.duration).clamp(0.0, 1.0);
        let eased_t = easing::ease_out_back(t);

        let one_minus_eased_t = 1.0 - eased_t;
        self.start * (one_minus_eased_t * one_minus_eased_t)
            + self.control * (2.0 * one_minus_eased_t * eased_t)
            + self.end * (eased_t * eased_t)
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}
