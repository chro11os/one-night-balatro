use raylib::prelude::*;
use crate::consts::*;

pub fn initialize_window() -> (RaylibHandle, RaylibThread) {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("One Night Balatro")
        .msaa_4x()
        .vsync() // NEW: Enables VSync
        .build();

    // With VSync enabled, this acts as a limiter cap,
    // but the monitor refresh rate usually takes priority.
    rl.set_target_fps(144);

    (rl, thread)
}