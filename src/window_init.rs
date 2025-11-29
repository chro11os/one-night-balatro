use raylib::prelude::*;
use crate::consts::*;

pub fn initialize_window() -> (RaylibHandle, RaylibThread) {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("One Night Balatro")
        .msaa_4x()
        .build();

    rl.set_target_fps(60);

    (rl, thread)
}