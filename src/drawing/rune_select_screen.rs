use raylib::prelude::*;
use crate::structures::stats::{BaseModifiers, RuneType};
use crate::structures::assets::GameAssets;
use crate::consts::*;
use crate::drawing::ui_elements::get_button_offset;

pub fn draw_rune_select(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    let center_x = SCREEN_WIDTH / 2.0;
    let content_offset = RUNE_CONTENT_OFFSET;
    let start_y_base = RUNE_START_Y;

    // FIX: Use Custom Font
    d.draw_text_ex(&assets.font_main, "CHOOSE YOUR PATH", Vector2::new(center_x + content_offset - 200.0, 50.0), 60.0, 1.0, NEU_ORANGE);
    d.draw_text_ex(&assets.font_main, "Select one rune from each row.", Vector2::new(center_x + content_offset - 220.0, 120.0), 24.0, 1.0, Color::GRAY);

    let rows = [
        (RuneType::Red, "COMBAT STYLE", NEU_RED, start_y_base),
        (RuneType::Blue, "UTILITY", NEU_BLUE, start_y_base + RUNE_SPACING_Y),
        (RuneType::Green, "ECONOMY", Color::new(76, 175, 80, 255), start_y_base + RUNE_SPACING_Y * 2.0),
        (RuneType::Minor, "STARTING BONUS", PARCHMENT, start_y_base + RUNE_SPACING_Y * 3.0),
    ];

    let mouse_pos = d.get_mouse_position();
    let mut hovered_rune_name = String::new();
    let mut hovered_rune_desc = String::new();
    let mut hovered_rune_color = Color::WHITE;

    for (r_type, label, color, y_pos) in rows.iter() {
        d.draw_text_ex(&assets.font_main, label, Vector2::new(50.0, *y_pos - 10.0), 20.0, 1.0, *color);

        let row_runes: Vec<_> = stats.available_runes.iter().filter(|r| r.rune_type == *r_type).collect();
        let count = row_runes.len();
        if count == 0 { continue; }

        let spacing = RUNE_SPACING_X;
        let row_width = (count as f32 - 1.0) * spacing;
        let start_x = (center_x + content_offset) - row_width / 2.0;

        for (i, rune) in row_runes.iter().enumerate() {
            let cx = start_x + (i as f32 * spacing);
            let cy = *y_pos;
            let radius = RUNE_RADIUS;

            let is_equipped = stats.equipped_runes.iter().any(|r| r.id == rune.id);
            let is_hovered = ((mouse_pos.x - cx).powi(2) + (mouse_pos.y - cy).powi(2)).sqrt() < radius;

            if is_equipped {
                d.draw_circle_lines(cx as i32, cy as i32, radius + 4.0, NEU_ORANGE);
                d.draw_circle(cx as i32, cy as i32, radius + 2.0, color.alpha(0.2));
            } else if is_hovered {
                d.draw_circle_lines(cx as i32, cy as i32, radius + 2.0, Color::WHITE);
            } else {
                d.draw_circle_lines(cx as i32, cy as i32, radius, color.alpha(0.5));
            }

            if let Some(tex) = assets.rune_icons.get(&rune.name) {
                let icon_size = radius * 2.0;
                let dest_rect = Rectangle::new(cx - radius, cy - radius, icon_size, icon_size);
                let src_rect = Rectangle::new(0.0, 0.0, tex.width as f32, tex.height as f32);
                let tint = if is_equipped || is_hovered { Color::WHITE } else { Color::GRAY };
                d.draw_texture_pro(tex, src_rect, dest_rect, Vector2::zero(), 0.0, tint);
            } else {
                d.draw_circle(cx as i32, cy as i32, radius, color.alpha(0.2));
                let letter = &rune.name[0..1];
                // FIX: Use Custom Font for rune letter
                d.draw_text_ex(&assets.font_main, letter, Vector2::new(cx - 10.0, cy - 15.0), 30.0, 1.0, PARCHMENT);
            }

            if is_hovered {
                hovered_rune_name = rune.name.clone();
                hovered_rune_desc = rune.description.clone();
                hovered_rune_color = *color;
            }
        }
    }

    if !hovered_rune_name.is_empty() {
        let info_x = SCREEN_WIDTH / 2.0 - 60.0;
        let info_y = 250.0;
        let info_w = 320.0;
        let info_h = 250.0;

        d.draw_rectangle_rounded(Rectangle::new(info_x, info_y, info_w, info_h), 0.1, 4, NEU_BLACK.alpha(0.9));
        d.draw_rectangle_rounded_lines_ex(Rectangle::new(info_x, info_y, info_w, info_h), 0.1, 4, 2.0, hovered_rune_color);

        d.draw_text_ex(&assets.font_main, &hovered_rune_name, Vector2::new(info_x + 20.0, info_y + 20.0), 30.0, 1.0, hovered_rune_color);
        d.draw_text_ex(&assets.font_main, "Effect:", Vector2::new(info_x + 20.0, info_y + 60.0), 20.0, 1.0, Color::GRAY);

        let max_text_width = info_w - 40.0;
        let font_size = 20.0; // Float for text_ex
        let mut current_y = info_y + 90.0;

        for paragraph in hovered_rune_desc.split('\n') {
            let words: Vec<&str> = paragraph.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                let test_line = if current_line.is_empty() { word.to_string() } else { format!("{} {}", current_line, word) };
                // FIX: Measure text using custom font
                if assets.font_main.measure_text(&test_line, font_size, 1.0).x > max_text_width {
                    d.draw_text_ex(&assets.font_main, &current_line, Vector2::new(info_x + 20.0, current_y), font_size, 1.0, PARCHMENT);
                    current_line = word.to_string();
                    current_y += 24.0;
                } else {
                    current_line = test_line;
                }
            }
            if !current_line.is_empty() {
                d.draw_text_ex(&assets.font_main, &current_line, Vector2::new(info_x + 20.0, current_y), font_size, 1.0, PARCHMENT);
                current_y += 24.0;
            }
        }
    }

    let panel_x = SCREEN_WIDTH - 420.0;
    let panel_w = 380.0;
    let panel_h = SCREEN_HEIGHT - 200.0;

    d.draw_rectangle_rounded(Rectangle::new(panel_x, 100.0, panel_w, panel_h), 0.05, 4, NEU_BLACK.alpha(0.8));
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(panel_x, 100.0, panel_w, panel_h), 0.05, 4, 2.0, NEU_ORANGE);

    d.draw_text_ex(&assets.font_main, "CURRENT LOADOUT", Vector2::new(panel_x + 80.0, 120.0), 24.0, 1.0, PARCHMENT);

    let mut benefits: Vec<String> = Vec::new();
    let mut downsides: Vec<String> = Vec::new();

    for rune in &stats.equipped_runes {
        let name = &rune.name;
        match name.as_str() {
            "Paladin" => {
                benefits.push(format!("- {}: +40 Max HP", name));
                downsides.push(format!("- {}: -10% Relic Mult", name));
            },
            "Reaper" => {
                benefits.push(format!("- {}: Lifesteal on Kill", name));
                downsides.push(format!("- {}: -15 Max HP", name));
            },
            "Judgement" => {
                benefits.push(format!("- {}: Balanced Calc", name));
                downsides.push(format!("- {}: Enemies 2x HP", name));
            },
            "Midas" => {
                benefits.push(format!("- {}: Gain Gold on Win", name));
                downsides.push(format!("- {}: Lose Gold on Loss", name));
            },
            "Greed" => {
                benefits.push(format!("- {}: +1 Hand, +1 Disc", name));
                downsides.push(format!("- {}: -1 Relic Slot", name));
            },
            "Investment" => {
                benefits.push(format!("- {}: +5% Interest", name));
                downsides.push(format!("- {}: Start with $0", name));
            },
            "Merchant" => {
                benefits.push(format!("- {}: +1 Shop Slot", name));
                downsides.push(format!("- {}: Prices +20%", name));
            },
            "Mentalist" => {
                benefits.push(format!("- {}: Free Scrolls", name));
                downsides.push(format!("- {}: Less Scrolls", name));
            },
            "Evolution" => {
                benefits.push(format!("- {}: Relics 1.5x Val", name));
                downsides.push(format!("- {}: Enemies 1.5x HP", name));
            },
            "Force" => benefits.push(format!("- {}: +10 Base Mult", name)),
            "Flow" => benefits.push(format!("- {}: +10 Base Chips", name)),
            "Wealth" => benefits.push(format!("- {}: +3 Gold/Round", name)),
            _ => {}
        }
    }

    let mut list_y = 170.0;

    d.draw_text_ex(&assets.font_main, "BENEFITS", Vector2::new(panel_x + 20.0, list_y), 20.0, 1.0, Color::GRAY);
    list_y += 30.0;
    for text in benefits {
        d.draw_text_ex(&assets.font_main, &text, Vector2::new(panel_x + 30.0, list_y), 20.0, 1.0, NEU_GREEN);
        list_y += 30.0;
    }

    list_y += 20.0;

    d.draw_text_ex(&assets.font_main, "TRADE-OFFS", Vector2::new(panel_x + 20.0, list_y), 20.0, 1.0, Color::GRAY);
    list_y += 30.0;
    for text in downsides {
        d.draw_text_ex(&assets.font_main, &text, Vector2::new(panel_x + 30.0, list_y), 20.0, 1.0, NEU_RED);
        list_y += 30.0;
    }

    let btn_w = 250.0;
    let btn_h = 70.0;
    let btn_x = center_x + content_offset - btn_w / 2.0;
    let btn_y = SCREEN_HEIGHT - 120.0;
    let btn_rect = Rectangle::new(btn_x, btn_y, btn_w, btn_h);
    let (off, shad) = get_button_offset(d, btn_rect);

    d.draw_rectangle_rounded(Rectangle::new(btn_x, btn_y + shad, btn_w, btn_h), 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(Rectangle::new(btn_x, btn_y + off, btn_w, btn_h), 0.2, 4, NEU_ORANGE);
    d.draw_text_ex(&assets.font_main, "START RUN", Vector2::new(btn_x + 55.0, btn_y + 20.0 + off), 28.0, 1.0, Color::BLACK);
}
