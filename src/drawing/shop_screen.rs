use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::assets::GameAssets;
use crate::consts::*;

pub fn draw_shop(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    let y_offset = stats.window_y_offset;
    d.draw_text_ex(&assets.font_main, "Shop", Vector2::new(SCREEN_WIDTH / 2.0 - 100.0, 50.0 + y_offset), 80.0, 1.0, PARCHMENT);
    d.draw_text_ex(&assets.font_main, &stats.money_text, Vector2::new(SCREEN_WIDTH - 300.0, 50.0 + y_offset), 30.0, 1.0, NEU_YELLOW);

    for (i, relic) in stats.current_shop_relics.iter().enumerate() {
        let x = SHOP_START_X + (i as f32 * (SHOP_ITEM_W + SHOP_ITEM_GAP));
        let y = SHOP_START_Y + y_offset;
        let rect = Rectangle::new(x, y, SHOP_ITEM_W, SHOP_ITEM_H);

        d.draw_rectangle_rounded(rect, 0.1, 4, NEU_BLACK.alpha(0.9));
        d.draw_rectangle_rounded_lines_ex(rect, 0.1, 4, 3.0, NEU_ORANGE);

        d.draw_text_ex(&assets.font_main, &relic.data.name, Vector2::new(x + 20.0, y + 20.0), 30.0, 1.0, PARCHMENT);

        // Text wrapping for description
        let max_text_width = SHOP_ITEM_W - 40.0;
        let font_size = 20.0;
        let mut current_y = y + 70.0;
        for paragraph in relic.data.description.split('\n') {
            let words: Vec<&str> = paragraph.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                let test_line = if current_line.is_empty() { word.to_string() } else { format!("{} {}", current_line, word) };
                if assets.font_main.measure_text(&test_line, font_size, 1.0).x > max_text_width {
                    d.draw_text_ex(&assets.font_main, &current_line, Vector2::new(x + 20.0, current_y), font_size, 1.0, Color::WHITE);
                    current_line = word.to_string();
                    current_y += 24.0;
                } else {
                    current_line = test_line;
                }
            }
            if !current_line.is_empty() {
                d.draw_text_ex(&assets.font_main, &current_line, Vector2::new(x + 20.0, current_y), font_size, 1.0, Color::WHITE);
                current_y += 24.0;
            }
        }

        d.draw_text_ex(&assets.font_main, &format!("Price: ${}", relic.data.value.unwrap_or(0)), Vector2::new(x + 20.0, y + SHOP_ITEM_H - 50.0), 24.0, 1.0, NEU_YELLOW);
    }

    let mut leave_btn = SHOP_LEAVE_BTN_RECT;
    leave_btn.y += y_offset;
    d.draw_rectangle_rec(leave_btn, NEU_RED);
    d.draw_text_ex(&assets.font_main, "Leave", Vector2::new(leave_btn.x + 60.0, leave_btn.y + 15.0), 24.0, 1.0, Color::WHITE);
}
