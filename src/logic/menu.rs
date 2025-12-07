use raylib::prelude::*;
use crate::structures::state::GameState;

pub fn update_menu(rl: &RaylibHandle, state: &mut GameState) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        *state = GameState::RuneSelect;
    }
}
