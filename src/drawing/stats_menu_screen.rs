use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::assets::GameAssets;
use crate::consts::*;

pub fn draw_stats_menu(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    let center_x = SCREEN_WIDTH / 2.0;
    let center_y = SCREEN_HEIGHT / 2.0;
    let box_w = 600.0;
    let box_h = 500.0;

    d.draw_rectangle_rounded(Rectangle::new(center_x - box_w/2.0, center_y - box_h/2.0, box_w, box_h), 0.05, 4, NEU_BLACK);
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(center_x - box_w/2.0, center_y - box_h/2.0, box_w, box_h), 0.05, 4, 3.0, NEU_ORANGE);

    // Close Button
    let close_btn_x = center_x + box_w/2.0 - 45.0;
    let close_btn_y = center_y - box_h/2.0 + 15.0;
    d.draw_rectangle(close_btn_x as i32, close_btn_y as i32, 30, 30, NEU_RED);
    d.draw_text_ex(&assets.font_main, "X", Vector2::new(close_btn_x + 8.0, close_btn_y + 5.0), 20.0, 1.0, Color::WHITE);

    d.draw_text_ex(&assets.font_main, "UPGRADES", Vector2::new(center_x - 100.0, center_y - box_h/2.0 + 30.0), 40.0, 1.0, NEU_ORANGE);
    d.draw_text_ex(&assets.font_main, &stats.stat_points_text, Vector2::new(center_x - 120.0, center_y - box_h/2.0 + 80.0), 30.0, 1.0, PARCHMENT);

    let start_y = center_y - 100.0;
    let row_h = 60.0;
    let btn_w = 120.0;
    let btn_h = 40.0;

    let stats_display = [
        ("Max HP", &stats.max_hp_stat_text),
        ("Crit Chance", &stats.crit_chance_stat_text),
        ("Crit Dmg", &stats.crit_mult_stat_text),
    ];

    for (i, (label, val)) in stats_display.iter().enumerate() {
        let y = start_y + (i as f32 * row_h);
        d.draw_text_ex(&assets.font_main, label, Vector2::new(center_x - 200.0, y), 30.0, 1.0, Color::WHITE);
        d.draw_text_ex(&assets.font_main, val, Vector2::new(center_x - 20.0, y), 30.0, 1.0, NEU_YELLOW);

        if stats.stat_points > 0 {
            let btn_rect = Rectangle::new(center_x + 100.0, y, btn_w, btn_h);
            d.draw_rectangle_rounded(btn_rect, 0.2, 4, NEU_GREEN);
            d.draw_text_ex(&assets.font_main, "+ UPGRADE", Vector2::new(btn_rect.x + 10.0, btn_rect.y + 10.0), 20.0, 1.0, Color::BLACK);
        }
    }
}
