use raylib::prelude::*;
use crate::structures::stats::{BaseModifiers, BossAbility};
use crate::structures::card::Card;
use crate::structures::state::GameState;
use crate::consts::*;

pub fn update_shop(rl: &mut RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers, _hand: &mut Vec<Card>, _deck: &mut Vec<Card>) {
    if stats.input_consumed {
        stats.input_consumed = false;
        return;
    }
    if stats.window_y_offset > 0.0 {
        stats.window_y_offset -= 1000.0 * rl.get_frame_time();
        if stats.window_y_offset < 0.0 {
            stats.window_y_offset = 0.0;
        }
    }

    if stats.current_shop_relics.is_empty() && stats.all_relics.len() > 0 {
        // Generate 3 random relics for the shop
        let mut available_relics = stats.all_relics.clone();
        // Filter out already equipped relics
        available_relics.retain(|r| !stats.equipped_relics.iter().any(|er| er.id == r.id));

        for _ in 0..3 {
            if available_relics.is_empty() {
                break;
            }
            let max_val = available_relics.len() as i32 - 1;
            if max_val < 0 { break; }
            let index = rl.get_random_value::<i32>(0..max_val) as usize;
            stats.current_shop_relics.push(available_relics.remove(index));
        }
    }

    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && stats.window_y_offset == 0.0 {
        let mouse_pos = rl.get_mouse_position();
        let mut purchased_relic_indices = Vec::new();

        // Check for clicks on relics
        for (i, relic) in stats.current_shop_relics.iter().enumerate() {
            let x = SHOP_START_X + (i as f32 * (SHOP_ITEM_W + SHOP_ITEM_GAP));
            let y = SHOP_START_Y;
            let rect = Rectangle::new(x, y, SHOP_ITEM_W, SHOP_ITEM_H);

            if rect.check_collision_point_rec(mouse_pos) {
                if stats.money >= relic.value {
                    stats.money -= relic.value;
                    if relic.name == "Bag of Holding" {
                        stats.hand_size += 1;
                        stats.discards_remaining += 1;
                    }
                    stats.equipped_relics.push(relic.clone());
                    purchased_relic_indices.push(i);
                }
            }
        }

        stats.current_shop_relics = stats.current_shop_relics.iter().enumerate()
            .filter(|(i, _)| !purchased_relic_indices.contains(i))
            .map(|(_, r)| r.clone())
            .collect();


        // Check for click on leave button
        if SHOP_LEAVE_BTN_RECT.check_collision_point_rec(mouse_pos) {
            stats.current_shop_relics.clear();
            // Reset for next round
            stats.round_score = 0;
            stats.round += 1;
            stats.target_score = (stats.target_score as f32 * 1.5) as i32; // Increase target score

            // Assign new enemy and ability
            if let Some(enemy_db) = &stats.enemy_database {
                let tier = if stats.round <= 3 { &enemy_db.tier_1 } else if stats.round <= 6 { &enemy_db.tier_2 } else { &enemy_db.tier_3 };
                if !tier.is_empty() {
                    let index = rl.get_random_value::<i32>(0..tier.len() as i32 - 1) as usize;
                    stats.enemy_name = tier[index].clone();
                }

                let ability_roll = rl.get_random_value::<i32>(0..5);
                stats.active_ability = match ability_roll {
                    1 => BossAbility::SilenceSuit(rl.get_random_value::<i32>(0..4)),
                    2 => BossAbility::HandSizeMinusOne,
                    3 => BossAbility::DoubleTarget,
                    4 => BossAbility::PayToDiscard,
                    _ => BossAbility::None,
                };
            }
            *state = GameState::Playing;
            stats.input_consumed = true;
            // Fading Torch effect
            if stats.equipped_relics.iter().any(|r| r.name == "Fading Torch") {
                for relic in &mut stats.all_relics {
                    if relic.name == "Fading Torch" {
                        relic.value -= 3;
                    }
                }
            }
        }
    }
}
