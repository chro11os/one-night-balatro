use crate::poker;
use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::card::Card;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;

// --- GAMEPLAY LOGIC ---
pub fn update_game(rl: &RaylibHandle, hand: &mut Vec<Card>, deck: &mut Vec<Card>, stats: &mut BaseModifiers, dt: f32, state: &mut GameState, animation_state: &mut AnimationState, total_time: f32) {
    let mouse_pos = rl.get_mouse_position();

    // ACTION STATES
    let mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_released = rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);

    // --- INTERACTION & DRAGGING ---
    if *animation_state == AnimationState::Idle {
        let mut hovered_index = None;
        let mut interaction_index = None; // Could be Pressed OR Dragging

        // 1. Find Hovered / Interacting Card
        // Check if we are already holding/dragging a card
        if let Some(idx) = hand.iter().position(|c| c.is_pressed || c.is_dragging) {
            interaction_index = Some(idx);
            hovered_index = Some(idx);
        } else {
            // Standard Hover Check
            for (i, card) in hand.iter_mut().enumerate().rev() {
                let width = CARD_WIDTH * card.target_scale;
                let height = CARD_HEIGHT * card.target_scale;
                let hit_x = card.target_pos.x;
                let hit_y = card.target_pos.y;
                let rect = Rectangle::new(hit_x - width / 2.0, hit_y - height / 2.0, width, height);

                if rect.check_collision_point_rec(mouse_pos) {
                    hovered_index = Some(i);
                    break;
                }
            }
        }

        // 2. Handle Inputs

        // CLICK DOWN (Prepare)
        if mouse_pressed {
            if let Some(idx) = hovered_index {
                let card = &mut hand[idx];
                card.is_pressed = true;
                card.is_dragging = false; // Not dragging yet!
                card.click_pos = mouse_pos; // Record where we started
            }
        }

        // MOUSE HOLD (Check Threshold or Update Drag)
        if mouse_down {
            if let Some(idx) = interaction_index {
                let card = &mut hand[idx];

                // THRESHOLD CHECK: Have we moved enough to call it a drag?
                if card.is_pressed && !card.is_dragging {
                    let distance = ((mouse_pos.x - card.click_pos.x).powi(2) + (mouse_pos.y - card.click_pos.y).powi(2)).sqrt();
                    if distance > 5.0 {
                        card.is_dragging = true;
                        // Visual pop now that we are definitely dragging
                        card.scale = SELECTED_SCALE * 1.1;
                    }
                }

                // DRAG LOGIC
                if card.is_dragging {
                    card.target_pos.x = mouse_pos.x;

                    if card.is_selected {
                        card.target_pos.y = HAND_Y_POS - 60.0;
                    } else {
                        card.target_pos.y = HAND_Y_POS;
                    }

                    // Swap Logic (Only when actually dragging)
                    if idx > 0 {
                        if hand[idx].target_pos.x < hand[idx - 1].target_pos.x {
                            hand.swap(idx, idx - 1);
                        }
                    }
                    if idx < hand.len() - 1 {
                        if hand[idx].target_pos.x > hand[idx + 1].target_pos.x {
                            hand.swap(idx, idx + 1);
                        }
                    }
                }
            }
        }

        // RELEASE (Decide: Click or Drop)
        if mouse_released {
            if let Some(idx) = interaction_index {
                let selected_count = hand.iter().filter(|c| c.is_selected).count();
                let card = &mut hand[idx];

                if card.is_dragging {
                    // We were dragging, so this is just a Drop
                    card.is_dragging = false;
                    card.is_pressed = false;
                } else if card.is_pressed {
                    // We were pressed but never moved far enough -> It's a CLICK
                    card.is_pressed = false;

                    // Toggle Selection
                    if card.is_selected {
                        card.is_selected = false;
                    } else if selected_count < 5 {
                        card.is_selected = true;
                    }
                }
            }
        }

        // Update Hover States
        for (i, card) in hand.iter_mut().enumerate() {
            let is_hovered = Some(i) == hovered_index;
            if card.is_hovered != is_hovered {
                card.is_hovered = is_hovered;
            }
        }
    }

    // --- POSITIONING ---
    calculate_hand_positions(hand, animation_state);

    // --- PHYSICS UPDATE ---
    for card in hand.iter_mut() {
        card.update_anim(dt, total_time);
    }

    // --- SCORING PREVIEW ---
    let selected_cards: Vec<Card> = hand.iter().filter(|c| c.is_selected).cloned().collect();
    if !selected_cards.is_empty() {
        let rank = poker::get_hand_rank(&selected_cards);
        stats.hand_rank = Some(rank);
        let (base_chips, base_mult) = poker::get_hand_base_score(rank);
        let mut card_chips = 0;
        for card in &selected_cards {
            card_chips += poker::get_card_chip_value(card);
        }
        stats.chips = base_chips + card_chips;
        stats.mult = base_mult;
    } else {
        stats.hand_rank = None;
        stats.chips = 0;
        stats.mult = 0;
    }

    // --- BUTTONS ---
    let center_x = SIDEBAR_WIDTH + (SCREEN_WIDTH - SIDEBAR_WIDTH)/2.0;
    let btn_y = 660.0;
    let sort_y = 620.0;

    let play_btn = Rectangle::new(center_x - BTN_WIDTH - 10.0, btn_y, BTN_WIDTH, BTN_HEIGHT);
    let discard_btn = Rectangle::new(center_x + 10.0, btn_y, BTN_WIDTH, BTN_HEIGHT);
    let sort_rank_btn = Rectangle::new(center_x - SORT_BTN_WIDTH - 5.0, sort_y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let sort_suit_btn = Rectangle::new(center_x + 5.0, sort_y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);

    if *animation_state == AnimationState::Idle && mouse_pressed {
        // Buttons Logic
        if play_btn.check_collision_point_rec(mouse_pos) && stats.hands_remaining > 0 && !selected_cards.is_empty() {
            *animation_state = AnimationState::Playing;
        }
        if discard_btn.check_collision_point_rec(mouse_pos) && stats.discards_remaining > 0 && !selected_cards.is_empty() {
            let old_len = hand.len();
            hand.retain(|c| !c.is_selected);
            let discarded_count = old_len - hand.len();
            if discarded_count > 0 {
                stats.discards_remaining -= 1;
                draw_cards(hand, deck, discarded_count);
            }
        }
        if sort_rank_btn.check_collision_point_rec(mouse_pos) {
            hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit)));
        }
        if sort_suit_btn.check_collision_point_rec(mouse_pos) {
            hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value)));
        }
    }

    // --- ANIMATION PHASES ---
    if *animation_state == AnimationState::Playing {
        let mut all_arrived = true;
        let played_count = hand.iter().filter(|c| c.is_selected).count();
        let played_spacing = 150.0;
        let total_w = (played_count as f32 - 1.0) * played_spacing;
        let start_x = center_x - total_w / 2.0;

        let mut idx = 0;
        for card in hand.iter_mut() {
            if card.is_selected {
                let target_x = start_x + (idx as f32 * played_spacing);
                card.target_pos = Vector2::new(target_x, PLAYED_Y_POS);
                card.target_scale = PLAYED_SCALE;
                card.target_rotation = 0.0;

                if (card.current_pos.y - PLAYED_Y_POS).abs() > 2.0 || (card.current_pos.x - target_x).abs() > 2.0 {
                    all_arrived = false;
                }
                idx += 1;
            }
        }

        if all_arrived {
            stats.hands_remaining -= 1;
            stats.total_score += stats.chips * stats.mult;
            stats.chips = 0;
            stats.mult = 0;
            stats.hand_rank = None;
            *animation_state = AnimationState::Scoring;
        }
    }

    if *animation_state == AnimationState::Scoring {
        let diff = stats.total_score as f32 - stats.display_score;
        if diff < 1.0 {
            stats.display_score = stats.total_score as f32;
            let mut all_shrunk = true;
            for card in hand.iter_mut() {
                if card.is_selected {
                    card.target_scale = 0.0;
                    if card.scale > 0.05 { all_shrunk = false; }
                }
            }
            if all_shrunk {
                hand.retain(|c| !c.is_selected);
                *animation_state = AnimationState::Idle;
                if stats.hands_remaining == 0 && stats.total_score < stats.target_score {
                    *state = GameState::GameOver;
                } else {
                    draw_cards(hand, deck, 8 - hand.len());
                }
            }
        } else {
            let speed = (diff * 5.0 * dt).max(10.0 * dt);
            stats.display_score += speed;
        }
    }

    stats.deck_count = deck.len() as i32;
}

fn calculate_hand_positions(hand: &mut Vec<Card>, anim_state: &AnimationState) {
    let count = hand.len();
    if count == 0 { return; }

    let center_x = SIDEBAR_WIDTH + (SCREEN_WIDTH - SIDEBAR_WIDTH) / 2.0;
    let hand_y = HAND_Y_POS;
    let card_spacing = 100.0;

    let total_hand_span = (count as f32 - 1.0) * card_spacing;
    let hand_start_x = center_x - total_hand_span / 2.0;

    for (i, card) in hand.iter_mut().enumerate() {
        // If dragging, let logic.rs handle X. We just set Y/Scale.
        if card.is_dragging {
            if card.is_selected {
                card.target_pos.y = hand_y - 60.0;
                card.target_scale = SELECTED_SCALE * 1.1;
            } else {
                card.target_pos.y = hand_y;
                card.target_scale = HAND_SCALE * 1.1;
            }
            continue;
        }

        if *anim_state != AnimationState::Idle && card.is_selected {
            continue;
        }

        card.target_pos.x = hand_start_x + (i as f32 * card_spacing);

        if card.is_selected {
            card.target_pos.y = hand_y - 60.0;
            card.target_scale = SELECTED_SCALE;
            card.target_rotation = 0.0;
        } else {
            card.target_pos.y = hand_y;
            card.target_scale = HAND_SCALE;
            card.target_rotation = 0.0;
        }
    }
}

pub fn update_menu(rl: &RaylibHandle, state: &mut GameState) {
    if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
        *state = GameState::Playing;
    }
}

fn draw_cards(hand: &mut Vec<Card>, deck: &mut Vec<Card>, count: usize) {
    for _ in 0..count {
        if let Some(mut card) = deck.pop() {
            card.current_pos = Vector2::new(DECK_X, DECK_Y);
            card.rotation = 1.0;
            hand.push(card);
        }
    }
}