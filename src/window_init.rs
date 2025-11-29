use raylib::prelude::*;

// Returns the Handle/Thread so main.rs can run the loop
pub fn initialize_window() -> (RaylibHandle, RaylibThread) {
    let screen_width = 1280;
    let screen_height = 720;
    let target_fps = 60;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("One Night Balatro - Optimized")
        .msaa_4x() // Smooths the rounded rectangles
        .build();

    rl.set_target_fps(target_fps);

    (rl, thread)
}