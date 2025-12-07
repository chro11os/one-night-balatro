pub fn ease_out_back(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    let t = t - 1.0;
    1.0 + c3 * t.powi(3) + c1 * t.powi(2)
}

pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t - 1.0;
    t.powi(3) + 1.0
}
