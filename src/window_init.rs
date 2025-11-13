use raylib::ffi::SetTargetFPS;
use raylib::prelude::*;
pub fn initialize_window()
{
    let screen_height:u16 = 410;
    let screen_width:u16 = 720;
    let target_fps:u8 = 60;

    let (mut rl, thread) = raylib::init()
        .size(screen_width as i32, screen_height as i32)
        .title("Test Window")
        .build();

    rl.set_target_fps(target_fps as u32);

    while !rl.window_should_close()
    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_fps(205,360);

        d.draw_text("One Night Balatro" ,300, 360,4, Color::BLACK);
        
    }
}