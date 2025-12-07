use crate::poker;
use raylib::prelude::*;
use crate::structures::stats::{self, BaseModifiers, BossAbility, SortMode};
use crate::structures::card::Card;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;
use crate::score_manager; // Now importing the module
use crate::structures::relic::{GameRelic, RelicEffect};
use crate::logic::metrics::GameMetrics; // Import GameMetrics

pub fn update_game(rl: &RaylibHandle, hand: &mut Vec<Card>, deck: &mut Vec<Card>, stats: &mut BaseModifiers, dt: f32, state: &mut GameState, animation_state: &mut AnimationState) { // Removed total_time

    if stats.input_consumed {
        stats.input_consumed = false;
        return;
    }
    let mouse_pos = rl.get_mouse_position();
    let mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_released = rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);

    let play_btn = Rectangle::new(PLAY_BTN_POS.x, PLAY_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);
    let discard_btn = Rectangle::new(DISC_BTN_POS.x, DISC_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);
    let sort_rank_btn = Rectangle::new(SORT_RANK_POS.x, SORT_RANK_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let sort_suit_btn = Rectangle::new(SORT_SUIT_POS.x, SORT_SUIT_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);

    // Update individual card tweens
    for card in hand.iter_mut() {
        if let Some(tween) = &mut card.tween {
            tween.update(dt);
            card.current_pos = tween.solve();

            if tween.is_finished() {
                card.current_pos = tween.end; // Snap to exact end to prevent drift
                card.tween = None;            // Animation complete
            }
        }
    }

    // Call the new hand positioning function
    update_hand_positions(hand);

    // stats.update_vfx(dt); // Keep this if it's still needed globally

    match animation_state {
        AnimationState::Idle => {
            if mouse_pressed && STATS_BTN_RECT.check_collision_point_rec(mouse_pos) {
                *state = GameState::StatsMenu;
                return;
            }

            let mut hovered_index = None;

            for (i, card) in hand.iter_mut().enumerate().rev() {
                let width = CARD_WIDTH * card.scale.x;
                let height = CARD_HEIGHT * card.scale.y;
                let rect = Rectangle::new(card.current_pos.x - width / 2.0, card.current_pos.y - height / 2.0, width, height);

                if rect.check_collision_point_rec(mouse_pos) {
                    hovered_index = Some(i);
                    break;
                }
            }

            for (i, card) in hand.iter_mut().enumerate() {
                card.is_hovered = Some(i) == hovered_index;

                if card.is_hovered && !card.is_selected && !card.is_pressed {
                    card.scale = Vector2::new(1.25, 1.25);
                    card.rotation = 0.0;
                } else {
                    card.scale = Vector2::new(1.0, 1.0);
                    card.rotation = 0.0;
                }
            }

            if mouse_pressed {
                if let Some(idx) = hovered_index {
                    let card = &mut hand[idx];
                    card.is_pressed = true;
                    card.click_pos = mouse_pos;
                }
            }

            if mouse_down {
                if let Some(idx) = hand.iter().position(|c| c.is_pressed) {
                    if !hand[idx].is_dragging {
                        let dx = mouse_pos.x - hand[idx].click_pos.x;
                        let dy = mouse_pos.y - hand[idx].click_pos.y;
                        if (dx * dx + dy * dy).sqrt() > 5.0 {
                            hand[idx].is_dragging = true;
                        }
                    }
                    if hand[idx].is_dragging {
                        hand[idx].current_pos = mouse_pos; // Instantly move to mouse position
                        if idx > 0 {
                            let left_x = hand[idx - 1].current_pos.x;
                            if mouse_pos.x < left_x { hand.swap(idx, idx - 1); }
                        }
                        if let Some(new_idx) = hand.iter().position(|c| c.is_pressed) {
                            if new_idx < hand.len() - 1 {
                                let right_x = hand[new_idx + 1].current_pos.x;
                                if mouse_pos.x > right_x { hand.swap(new_idx, new_idx + 1); }
                            }
                        }
                    }
                }
            }

            if mouse_released {
                let selected_count = hand.iter().filter(|c| c.is_selected).count();
                for card in hand.iter_mut() {
                    if card.is_pressed && !card.is_dragging {
                        if !card.is_selected && selected_count >= 5 {
                            // Locked
                        } else {
                            card.is_selected = !card.is_selected;
                            // Squash and stretch effect (instantaneous for now)
                            if card.is_selected {
                                card.scale = Vector2::new(1.1, 0.9);
                            } else {
                                card.scale = Vector2::new(0.9, 1.1);
                            }
                            // Log the click
                            stats.game_metrics.log_click(card.id as usize);
                        }
                    }
                    card.is_pressed = false;
                    card.is_dragging = false;
                }
            }

            if mouse_pressed {
                let selected_count = hand.iter().filter(|c| c.is_selected).count();

                if play_btn.check_collision_point_rec(mouse_pos) && stats.hands_remaining > 0 && selected_count > 0 && selected_count <= 5 {
                    *animation_state = AnimationState::Playing;
                    stats.score_timer = 0.0;
                    stats.score_index = 0;
                }

                if discard_btn.check_collision_point_rec(mouse_pos) && stats.discards_remaining > 0 && selected_count > 0 && selected_count <= 5 {
                    stats.discards_remaining -= 1;
                    stats.game_metrics.log_discard(selected_count);
                    *animation_state = AnimationState::Discarding;
                    stats.discard_timer = 0.0;
                    stats.discard_index = 0;
                }

                if sort_rank_btn.check_collision_point_rec(mouse_pos) {
                    stats.current_sort = SortMode::Rank;
                    sort_hand(hand, SortMode::Rank);
                }
                if sort_suit_btn.check_collision_point_rec(mouse_pos) {
                    stats.current_sort = SortMode::Suit;
                    sort_hand(hand, SortMode::Suit);
                }
            }
        },

        AnimationState::Dealing => {
            // Future implementation
        },

        AnimationState::Discarding => {
            // stats.discard_timer += dt; // Not needed
            // let discard_delay = 0.15; // Not needed

            let mut selected_indices: Vec<usize> = hand.iter().enumerate()
                .filter(|(_, c)| c.is_selected)
                .map(|(i, _)| i)
                .collect();
            
            // Sort indices to discard from right to left, preventing index shifting issues
            selected_indices.sort_by(|a, b| b.cmp(a));


            if stats.discard_index < selected_indices.len() {
                // Instantly discard all selected cards
                for card_hand_index in selected_indices.iter() {
                    let card = &mut hand[*card_hand_index];
                    card.current_pos = Vector2::new(card.current_pos.x, SCREEN_HEIGHT as f32 + 200.0); // Instantly move off-screen
                    card.rotation = -0.3; // Instantly set rotation
                }
                stats.discard_index = selected_indices.len(); // All processed
            }
            
            // No waiting for animations to complete, cards are instantly gone
            // let all_gone = hand.iter().filter(|c| c.is_selected).all(|c| !c.pos_motion.active && c.current_pos.y > SCREEN_HEIGHT as f32 + 100.0);
            
            let discarded_count = hand.iter().filter(|c| c.is_selected).count();
            hand.retain(|c| !c.is_selected); // Instantly remove

            // "On Discard" effects
            if stats.equipped_relics.iter().any(|r| r.name == "Recycler") {
                stats.money += discarded_count as i32;
            }

            while hand.len() < stats.hand_size as usize {
                if let Some(mut new_card) = deck.pop() {
                    new_card.current_pos = Vector2::new(DECK_X, DECK_Y);
                    hand.push(new_card);
                } else { break; }
            }
            sort_hand(hand, stats.current_sort);
            *animation_state = AnimationState::Idle;
        },

        AnimationState::Playing => {
            // PHASE 1: Move Cards to Center (Tweened)
            let center_x = SCREEN_WIDTH / 2.0;
            let selected_count = hand.iter().filter(|c| c.is_selected).count();
            let spacing = 120.0;
            let start_x = center_x - ((selected_count as f32 - 1.0) * spacing) / 2.0;

            let mut idx = 0;
            let animation_duration = 0.4;
            for card in hand.iter_mut() {
                if card.is_selected {
                    let end_pos = Vector2::new(start_x + (idx as f32 * spacing), PLAY_AREA_Y);

                    // Instantaneously set position and scale
                    card.current_pos = end_pos;
                    card.scale = Vector2::new(1.1, 1.1);
                    
                    idx += 1;
                }
            }

            *animation_state = AnimationState::ScoringSeq;
            stats.score_timer = 0.0;
            stats.score_index = 0;
        },

        AnimationState::ScoringSeq => {
            // PHASE 2: Pop Cards Individually (Instantly)
            // stats.score_timer += dt; // Not needed
            // let step_delay = 0.3; // Not needed

            let selected_indices: Vec<usize> = hand.iter().enumerate()
                .filter(|(_, c)| c.is_selected)
                .map(|(i, _)| i)
                .collect();

            if stats.score_index < selected_indices.len() {
                // if stats.score_timer > step_delay { // Not needed, instant
                    let idx = selected_indices[stats.score_index];

                    // Visual Pop (Instantaneous)
                    hand[idx].scale = Vector2::new(1.6, 1.6);

                    // Add Chips Logic
                    let chips = poker::get_card_chip_value(&hand[idx]);
                    stats.chips += chips;

                    // Spawn VFX (This is still animation, but it's external to card motion. Keep for now or simplify if truly nuclear)
                    let pos = hand[idx].current_pos;
                    stats::spawn_floating_text(stats, format!("+{}", chips), pos - Vector2::new(0.0, 90.0), NEU_BLUE);
                    stats::spawn_particle_burst(stats, pos, NEU_YELLOW);

                    stats.score_index += 1;
                    stats.score_timer = 0.0;
                // }
            } else {
                // if stats.score_timer > 0.5 { // Not needed, instant
                    *animation_state = AnimationState::Scoring;
                    stats.score_timer = 0.0;
                // }
            }
        },

        AnimationState::Scoring => {
            // PHASE 3: Tally Total & Cleanup
            let selected: Vec<Card> = hand.iter().filter(|c| c.is_selected).cloned().collect();
            let rank = poker::get_hand_rank(&selected, stats);
            // Convert equipped_relics (RelicData) to GameRelic and ensure they have effects assigned
            let game_relics: Vec<GameRelic> = stats.equipped_relics.iter().map(|r_data| GameRelic { data: r_data.clone(), effect: RelicEffect::None /* FIX: This needs to be loaded from data */ }).collect();
            
            let (base_chips, base_mult) = poker::get_hand_base_score(rank);
            let score_result = score_manager::calculate_score(&selected, &game_relics, base_chips, base_mult);

            stats.total_score += score_result.total;
            stats.round_score += score_result.total;
            stats.game_metrics.log_play(score_result.total);

            // Trigger HP Damage Flash (still visual effect, keep or remove depending on strictness of "ZERO animations")
            // For now, setting it instantly or removing the effect entirely. I'll remove the flash.
            stats.display_score += score_result.total as f32;
            // stats.damage_flash_timer = 0.25; // Removed

            // Remove Cards & Decrement Hands
            hand.retain(|c| !c.is_selected);
            stats.hands_remaining -= 1;

            // CHECK FOR WIN/LOSS
            if stats.round_score >= stats.target_score {
                stats.window_y_offset = SCREEN_HEIGHT;
                *state = GameState::Shop;
                stats.money += 10;
                return;
            } else if stats.hands_remaining == 0 {
                *state = GameState::GameOver;
                return;
            }

            // Refill Hand
            let mut hand_size = stats.hand_size;
            if stats.active_ability == BossAbility::HandSizeMinusOne {
                hand_size -= 1;
            }
            while hand.len() < hand_size as usize {
                if let Some(mut new_card) = deck.pop() {
                    new_card.current_pos = Vector2::new(DECK_X, DECK_Y);
                    hand.push(new_card);
                } else { break; }
            }

            // Auto Sort
            sort_hand(hand, stats.current_sort);

            *animation_state = AnimationState::Idle;
        }
    }
}

fn sort_hand(hand: &mut Vec<Card>, mode: SortMode) {
    match mode {
        SortMode::Rank => hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit))),
        SortMode::Suit => hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value))),
    }
}

fn update_hand_positions(hand: &mut Vec<Card>) {
    let num_cards = hand.len();
    if num_cards == 0 { return; }

    // 1. Center the hand horizontally
    let left_bound = P_PANEL_X + P_PANEL_W + 50.0;
    let right_bound = A_PANEL_X - 50.0;
    let available_width = right_bound - left_bound;
    let center_x = left_bound + (available_width / 2.0);
    
    let spacing = 90.0; // Tighter spacing
    let start_x = center_x - ((num_cards as f32 - 1.0) * spacing) / 2.0;

    // 2. Set Vertical Position (Lower than before)
    // SCREEN_HEIGHT is 900, so -100 puts it at 800 (very close to bottom)
    let base_y = SCREEN_HEIGHT - 120.0; 

    for (i, card) in hand.iter_mut().enumerate() {
        // If dragging, skip auto-positioning
        if card.is_dragging { continue; }

        let target_x = start_x + (i as f32 * spacing);
        
        // 3. Animation Logic (Pop UP)
        // Subtracting Y moves the card UP visually
        let y_offset = if card.is_selected {
            60.0 // Pop up significantly
        } else if card.is_hovered {
            30.0 // Pop up slightly
        } else {
            0.0
        };

        let target_y = base_y - y_offset; 

        // Only tween if the position has changed significantly (Optimization)
        if card.target_pos.distance_to(Vector2::new(target_x, target_y)) > 1.0 {
            card.move_to(Vector2::new(target_x, target_y), 0.25); // Fast snap (0.25s)
            card.target_pos = Vector2::new(target_x, target_y);
        }
    }
}