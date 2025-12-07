use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::state::GameState;
use crate::consts::*;

pub fn update_rune_select(rl: &RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers) {
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let center_x = SCREEN_WIDTH / 2.0;

    let content_offset = RUNE_CONTENT_OFFSET;
    let start_y_base = RUNE_START_Y;

    let rows = [
        (crate::structures::stats::RuneType::Red, start_y_base),
        (crate::structures::stats::RuneType::Blue, start_y_base + RUNE_SPACING_Y),
        (crate::structures::stats::RuneType::Green, start_y_base + RUNE_SPACING_Y * 2.0),
        (crate::structures::stats::RuneType::Minor, start_y_base + RUNE_SPACING_Y * 3.0),
    ];

    if clicked {
        for (r_type, y_pos) in rows.iter() {
            let row_runes: Vec<_> = stats.available_runes.iter().filter(|r| r.rune_type == *r_type).collect();
            let count = row_runes.len();
            if count == 0 { continue; }

            let spacing = RUNE_SPACING_X;
            let row_width = (count as f32 - 1.0) * spacing;
            let start_x = (center_x + content_offset) - row_width / 2.0;

            for (i, rune) in row_runes.iter().enumerate() {
                let x = start_x + (i as f32 * spacing);
                let y = *y_pos;
                let dist = ((mouse_pos.x - x).powi(2) + (mouse_pos.y - y).powi(2)).sqrt();
                if dist < RUNE_RADIUS {
                    stats.equipped_runes.retain(|r| r.rune_type != *r_type);
                    stats.equipped_runes.push((*rune).clone());
                }
            }
        }

        let btn_w = 250.0;
        let btn_h = 70.0;
        let btn_x = center_x + content_offset - btn_w / 2.0;
        let btn_y = SCREEN_HEIGHT - 120.0;
        let btn_rect = Rectangle::new(btn_x, btn_y, btn_w, btn_h);

        if btn_rect.check_collision_point_rec(mouse_pos) {
            stats.shop_price_mult = 1.0;
            stats.ante_scaling = 1.5;
            stats.stat_points = 3;

            for rune in &stats.equipped_runes {
                match rune.name.as_str() {
                    "Paladin" => { stats.max_hp += 40; stats.current_hp += 40; },
                    "Reaper" => {
                        stats.max_hp -= 15;
                        if stats.max_hp < 1 { stats.max_hp = 1; }
                        stats.current_hp = stats.max_hp;
                    },
                    "Judgement" => stats.ante_scaling = 2.0,
                    "Greed" => { stats.hands_remaining += 1; stats.discards_remaining += 1; },
                    "Investment" => stats.money = 0,
                    "Merchant" => stats.shop_price_mult = 1.2,
                    "Evolution" => stats.ante_scaling = 2.25,
                    "Force" => stats.mult += 10,
                    "Flow" => stats.chips += 10,
                    "Wealth" => stats.money += 3,
                    _ => {}
                }
            }
            *state = GameState::Playing;
        }
    }
}
