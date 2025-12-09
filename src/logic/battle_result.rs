use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::state::GameState;
use crate::consts::*;

// Logic to calculate rewards
pub fn calculate_rewards(stats: &BaseModifiers) -> (i32, i32, i32) {
    let base_money = 4;
    let money_per_hand = 1;

    // Interest: $1 for every $5 held, capped at interest_cap/5
    let interest = std::cmp::min(stats.money / 5, stats.interest_cap / 5);
    let hands_bonus = stats.hands_remaining * money_per_hand;

    (base_money, interest, hands_bonus)
}

pub fn update_battle_result(rl: &mut RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers) {
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    let btn_w = 200.0;
    let btn_h = 60.0;
    let btn_x = SCREEN_WIDTH / 2.0 - btn_w / 2.0;
    let btn_y = SCREEN_HEIGHT / 2.0 + 100.0;
    let next_btn = Rectangle::new(btn_x, btn_y, btn_w, btn_h);

    if clicked && next_btn.check_collision_point_rec(mouse_pos) {
        // Apply Rewards
        let (base, interest, hands) = calculate_rewards(stats);
        stats.money += base + interest + hands;

        // Initialize Shop and Transition
        crate::logic::shop::init_shop(stats);
        *state = GameState::Shop;
    }
}