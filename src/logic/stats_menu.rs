use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::state::GameState;
use crate::consts::*;

pub fn update_stats_menu(rl: &RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers) {
    if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
        *state = GameState::Playing;
    }

    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    let center_x = SCREEN_WIDTH / 2.0;
    let center_y = SCREEN_HEIGHT / 2.0;
    let box_w = 600.0;
    let box_h = 500.0;
    let start_y = center_y - 100.0;

    let close_btn_x = center_x + box_w/2.0 - 45.0;
    let close_btn_y = center_y - box_h/2.0 + 15.0;
    let close_btn = Rectangle::new(close_btn_x, close_btn_y, 30.0, 30.0);

    if clicked && close_btn.check_collision_point_rec(mouse_pos) {
        *state = GameState::Playing;
    }

    if stats.stat_points > 0 && clicked {
        let btn_w = 120.0;
        let btn_h = 40.0;
        let btn_x = center_x + 100.0;

        let rect_hp = Rectangle::new(btn_x, start_y, btn_w, btn_h);
        let rect_crit = Rectangle::new(btn_x, start_y + 60.0, btn_w, btn_h);
        let rect_dmg = Rectangle::new(btn_x, start_y + 120.0, btn_w, btn_h);

        if rect_hp.check_collision_point_rec(mouse_pos) {
            stats.max_hp += 10;
            stats.current_hp += 10;
            stats.stat_points -= 1;
        } else if rect_crit.check_collision_point_rec(mouse_pos) {
            stats.crit_chance += 0.05;
            stats.stat_points -= 1;
        } else if rect_dmg.check_collision_point_rec(mouse_pos) {
            stats.crit_mult += 0.5;
            stats.stat_points -= 1;
        }
    }
}
