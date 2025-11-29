use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::card::Card;
use crate::consts::*;

// Note: We now take '&mut Vec<Card>' for hand and deck because we need to push/pop
pub fn update_game(rl: &RaylibHandle, hand: &mut Vec<Card>, deck: &mut Vec<Card>, stats: &mut BaseModifiers, dt: f32) {
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    // 1. Handle Card Selection
    for card in hand.iter_mut() {
        let rect = Rectangle::new(
            card.current_pos.x - CARD_WIDTH / 2.0,
            card.current_pos.y - CARD_HEIGHT / 2.0,
            CARD_WIDTH,
            CARD_HEIGHT
        );
        card.is_hovered = rect.check_collision_point_rec(mouse_pos);

        if card.is_hovered && clicked {
            if card.target_pos.y == HAND_Y_POS {
                card.target_pos.y = SELECTED_Y_POS; // Select
            } else {
                card.target_pos.y = HAND_Y_POS;     // Deselect
            }
        }
        card.update_anim(dt);
    }

    // 2. Define Buttons
    let play_rect = Rectangle::new(PLAY_BTN_POS.x, PLAY_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);
    let disc_rect = Rectangle::new(DISC_BTN_POS.x, DISC_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);

    // 3. Handle Play
    if clicked && play_rect.check_collision_point_rec(mouse_pos) && stats.hands_remaining > 0 {
        // Find selected cards
        // Note: We use retain to remove "Played" cards. 
        // Logic: Keep cards that are NOT selected.
        let old_len = hand.len();
        hand.retain(|c| c.target_pos.y == HAND_Y_POS);
        let played_count = old_len - hand.len();

        if played_count > 0 {
            stats.hands_remaining -= 1;
            stats.total_score += 100 * played_count as i32; // Dummy Scoring
            draw_cards(hand, deck, played_count);
        }
    }

    // 4. Handle Discard
    if clicked && disc_rect.check_collision_point_rec(mouse_pos) && stats.discards_remaining > 0 {
        let old_len = hand.len();
        hand.retain(|c| c.target_pos.y == HAND_Y_POS); // Remove selected
        let discarded_count = old_len - hand.len();

        if discarded_count > 0 {
            stats.discards_remaining -= 1;
            draw_cards(hand, deck, discarded_count);
        }
    }

    // Update Deck Count
    stats.deck_count = deck.len() as i32;
}

// Helper: Refills hand and Recalculates Positions
fn draw_cards(hand: &mut Vec<Card>, deck: &mut Vec<Card>, count: usize) {
    // 1. Draw new cards
    for _ in 0..count {
        if let Some(mut card) = deck.pop() {
            // Start animation from Deck Position
            card.current_pos = Vector2::new(DECK_X, DECK_Y);
            card.target_pos.y = HAND_Y_POS; // Reset selection state
            hand.push(card);
        }
    }

    // 2. Recenter Hand (Dynamic Layout)
    let hand_size = hand.len();
    if hand_size == 0 { return; }

    let gap = 10.0;
    let total_width = (hand_size as f32 * CARD_WIDTH) + ((hand_size - 1) as f32 * gap);
    let start_x = (SCREEN_WIDTH / 2.0) - (total_width / 2.0) + (CARD_WIDTH / 2.0);

    for (i, card) in hand.iter_mut().enumerate() {
        let dest_x = start_x + (i as f32 * (CARD_WIDTH + gap));
        card.target_pos.x = dest_x;
        // Ensure they aren't floating in "Selected" state
        card.target_pos.y = HAND_Y_POS;
    }
}