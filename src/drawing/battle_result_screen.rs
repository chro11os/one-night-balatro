use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::assets::GameAssets;
use crate::consts::*;

pub fn draw_battle_result(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    let y_offset = stats.window_y_offset;
    let rect = Rectangle::new(SCREEN_WIDTH / 2.0 - 200.0, SCREEN_HEIGHT / 2.0 - 100.0 + y_offset, 400.0, 200.0);
    d.draw_rectangle_rounded(rect, 0.1, 4, NEU_BLACK.alpha(0.9));
    d.draw_rectangle_rounded_lines_ex(rect, 0.1, 4, 3.0, NEU_ORANGE);
    d.draw_text_ex(&assets.font_main, "Battle Result", Vector2::new(rect.x + 50.0, rect.y + 50.0), 40.0, 1.0, PARCHMENT);
    d.draw_text_ex(&assets.font_main, "Click to continue", Vector2::new(rect.x + 100.0, rect.y + 120.0), 20.0, 1.0, Color::WHITE);
}
