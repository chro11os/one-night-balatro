use raylib::prelude::*;
use crate::structures::assets::GameAssets;
use crate::consts::*;

pub fn draw_menu(d: &mut RaylibDrawHandle, assets: &GameAssets) {
    let center_x = SCREEN_WIDTH / 2.0;
    let center_y = SCREEN_HEIGHT / 2.0;

    // Title
    let title = "ONE NIGHT BALATRO";
    let title_size = 80.0; // Must be f32 for draw_text_ex
    let title_dim = assets.font_main.measure_text(title, title_size, 1.0);

    d.draw_text_ex(
        &assets.font_main,
        title,
        Vector2::new(center_x - title_dim.x / 2.0, center_y - 150.0),
        title_size,
        1.0,
        PARCHMENT
    );

    // Subtitle
    let sub = "Click to Start";
    let sub_size = 40.0;
    let sub_dim = assets.font_main.measure_text(sub, sub_size, 1.0);

    d.draw_text_ex(
        &assets.font_main,
        sub,
        Vector2::new(center_x - sub_dim.x / 2.0, center_y + 100.0),
        sub_size,
        1.0,
        Color::GRAY
    );
}
