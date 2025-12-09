use raylib::prelude::*;
use rand::{self, Rng, seq::SliceRandom};
use crate::structures::stats::BaseModifiers;
use crate::structures::state::GameState;
use crate::structures::card::Card;
use crate::structures::relic::GameRelic;
use crate::consts::*;

// --- INITIALIZATION ---
pub fn init_shop(stats: &mut BaseModifiers) {
    // 1. Reset Animation (Start off-screen at bottom)
    stats.shop_y_offset = SCREEN_HEIGHT;

    // 2. Clear previous shop inventory
    stats.current_shop_relics.clear();

    // 3. Roll for new items (3 Slots)
    let num_slots = 3;
    let mut rng = rand::thread_rng();

    for _ in 0..num_slots {
        // --- THE D20 MECHANIC ---
        let roll = rng.gen_range(1..=20);
        let target_rarity = roll_rarity(roll);

        println!("Shop Roll: {} -> Looking for {}", roll, target_rarity);

        // Filter the Database by Rarity
        // We look at all_relics (loaded in stats)
        let pool: Vec<&GameRelic> = stats.all_relics.values()
            .filter(|r| r.data.rarity == target_rarity)
            .collect();

        // Fallback: If we rolled "Mythic" but have none in the DB, show "Common"
        let final_pool = if pool.is_empty() {
            stats.all_relics.values()
                .filter(|r| r.data.rarity == "Common")
                .collect()
        } else {
            pool
        };

        // Pick a random item from the pool
        if let Some(relic) = final_pool.choose(&mut rng) {
            stats.current_shop_relics.push((*relic).clone());
        }
    }
}

// Helper: The D20 Table
fn roll_rarity(roll: i32) -> String {
    match roll {
        1..=10 => "Common".to_string(),      // 50%
        11..=15 => "Uncommon".to_string(),   // 25%
        16..=18 => "Rare".to_string(),       // 15%
        19 => "Legendary".to_string(),       // 5%
        20 => "Mythic".to_string(),          // 5% (Natural 20!)
        _ => "Common".to_string(),
    }
}

// --- UPDATE LOOP ---
pub fn update_shop(rl: &mut RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers, deck: &mut Vec<Card>) {
    let dt = rl.get_frame_time();
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    // 1. Slide Animation (Slide Up to 0)
    if stats.shop_y_offset > 0.0 {
        stats.shop_y_offset -= 1500.0 * dt; // Fast slide
        if stats.shop_y_offset < 0.0 {
            stats.shop_y_offset = 0.0;
        }
    }

    // 2. Next Round Button Logic
    // Located at bottom right, moves with the panel
    let btn_w = 200.0;
    let btn_h = 60.0;
    let btn_x = SCREEN_WIDTH - btn_w - 50.0;
    let btn_y = SCREEN_HEIGHT - 100.0 + stats.shop_y_offset; // Follows animation
    let next_rect = Rectangle::new(btn_x, btn_y, btn_w, btn_h);

    if clicked && next_rect.check_collision_point_rec(mouse_pos) {
        // Start the next fight
        crate::logic::game::start_next_round(stats, deck);
        *state = GameState::Playing;
    }

    // 3. Buying Logic (Placeholder)
    // Here you would check clicks on the relics inside stats.current_shop_relics
    // and deduct stats.money if clicked.
}