use crate::poker;
use raylib::prelude::*;
use crate::structures::stats::{self, BaseModifiers, BossAbility, SortMode};
use crate::structures::card::Card;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;


pub fn update_game(rl: &RaylibHandle, hand: &mut Vec<Card>, deck: &mut Vec<Card>, stats: &mut BaseModifiers, dt: f32, state: &mut GameState, animation_state: &mut AnimationState, total_time: f32) {
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

    // Update VFX Physics
    stats.update_vfx(dt);

    match animation_state {
        AnimationState::Idle => {
            if mouse_pressed && STATS_BTN_RECT.check_collision_point_rec(mouse_pos) {
                *state = GameState::StatsMenu;
                return;
            }

            let mut hovered_index = None;

            for (i, card) in hand.iter_mut().enumerate().rev() {
                let width = CARD_WIDTH * card.target_scale.x;
                let height = CARD_HEIGHT * card.target_scale.y;
                let rect = Rectangle::new(card.current_pos.x - width / 2.0, card.current_pos.y - height / 2.0, width, height);

                if rect.check_collision_point_rec(mouse_pos) {
                    hovered_index = Some(i);
                    break;
                }
            }

            for (i, card) in hand.iter_mut().enumerate() {
                card.is_hovered = Some(i) == hovered_index;

                // Use tweening for hover/unhover scale/rotation
                if card.is_hovered && !card.is_selected && !card.is_pressed {
                    card.set_target_scale_rotation_tweened(Vector2::new(1.25, 1.25), (total_time * 6.0).sin() * 0.05, 0.1);
                } else if !card.is_dragging { // Only reset if not dragging
                    card.set_target_scale_rotation_tweened(Vector2::new(1.0, 1.0), 0.0, 0.2);
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
                        hand[idx].target_pos = mouse_pos;
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
                            // Apply a squash and stretch effect when selected/deselected
                            card.apply_squash_stretch(0.1, 0.2);
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
            stats.discard_timer += dt;
            let discard_delay = 0.15;

            let mut selected_indices: Vec<usize> = hand.iter().enumerate()
                .filter(|(_, c)| c.is_selected)
                .map(|(i, _)| i)
                .collect();
            
            // Sort indices to discard from right to left, preventing index shifting issues
            selected_indices.sort_by(|a, b| b.cmp(a));


            if stats.discard_index < selected_indices.len() {
                if stats.discard_timer > discard_delay {
                    let card_hand_index = selected_indices[stats.discard_index];

                    // Make the card fly off-screen with tweening
                    let card = &mut hand[card_hand_index];
                    card.set_target_pos_tweened(Vector2::new(card.current_pos.x, SCREEN_HEIGHT as f32 + 200.0), 0.5);
                    card.set_target_scale_rotation_tweened(card.target_scale, -0.3, 0.5); // Add some spin

                    stats.discard_index += 1;
                    stats.discard_timer = 0.0;
                }
            } else {
                // All cards have been sent flying, now wait for their animations to complete before removing
                let all_gone = hand.iter().filter(|c| c.is_selected).all(|c| !c.pos_motion.active && c.current_pos.y > SCREEN_HEIGHT as f32 + 100.0);

                if all_gone {
                    let discarded_count = hand.iter().filter(|c| c.is_selected).count();
                    hand.retain(|c| !c.is_selected);
    
                    // "On Discard" effects
                    if stats.equipped_relics.iter().any(|r| r.name == "Recycler") {
                        stats.money += discarded_count as i32;
                    }
    
                    while hand.len() < stats.hand_size as usize {
                        if let Some(mut new_card) = deck.pop() {
                            new_card.current_pos = Vector2::new(DECK_X, DECK_Y);
                            new_card.target_pos = Vector2::new(DECK_X, DECK_Y);
                            hand.push(new_card);
                        } else { break; }
                    }
                    sort_hand(hand, stats.current_sort);
                    *animation_state = AnimationState::Idle;
                }
            }
        },

        AnimationState::Playing => {
            // PHASE 1: Move Cards to Center
            let center_x = SCREEN_WIDTH / 2.0;
            let selected_count = hand.iter().filter(|c| c.is_selected).count();
            let spacing = 120.0;
            let start_x = center_x - ((selected_count as f32 - 1.0) * spacing) / 2.0;

            let mut idx = 0;
            let animation_duration = 0.4; // Slightly longer for arc animation
            for card in hand.iter_mut() {
                if card.is_selected {
                    let start_pos = card.current_pos;
                    let end_pos = Vector2::new(start_x + (idx as f32 * spacing), PLAY_AREA_Y);
                    let control_point = Vector2::new((start_pos.x + end_pos.x) / 2.0, (start_pos.y + end_pos.y) / 2.0 - 100.0);
                    
                    card.set_target_pos_bezier_tweened(end_pos, control_point, animation_duration);
                    card.set_target_scale_rotation_tweened(Vector2::new(1.1, 1.1), 0.0, animation_duration); // Slightly larger and no rotation
                    idx += 1;
                }
            }

            let mut all_arrived = true;
            for card in hand.iter() {
                if card.is_selected && card.is_moving() { // Check if the card is still animating or far from target
                    all_arrived = false;
                    break;
                }
            }

            if all_arrived {
                *animation_state = AnimationState::ScoringSeq;
                stats.score_timer = 0.0;
                stats.score_index = 0;
            }
        },

        AnimationState::ScoringSeq => {
            // PHASE 2: Pop Cards Individually
            stats.score_timer += dt;
            let step_delay = 0.3;

            let selected_indices: Vec<usize> = hand.iter().enumerate()
                .filter(|(_, c)| c.is_selected)
                .map(|(i, _)| i)
                .collect();

            if stats.score_index < selected_indices.len() {
                if stats.score_timer > step_delay {
                    let idx = selected_indices[stats.score_index];

                    // Visual Pop
                    hand[idx].scale = Vector2::new(1.6, 1.6);

                    // Add Chips Logic
                    let chips = poker::get_card_chip_value(&hand[idx]);
                    stats.chips += chips;

                    // Spawn VFX
                    let pos = hand[idx].current_pos;
                    stats::spawn_floating_text(stats, format!("+{}", chips), pos - Vector2::new(0.0, 90.0), NEU_BLUE);
                    stats::spawn_particle_burst(stats, pos, NEU_YELLOW);

                    stats.score_index += 1;
                    stats.score_timer = 0.0;
                }
            } else {
                if stats.score_timer > 0.5 {
                    *animation_state = AnimationState::Scoring;
                    stats.score_timer = 0.0;
                }
            }
        },

        AnimationState::Scoring => {
            // PHASE 3: Tally Total & Cleanup
            let selected: Vec<Card> = hand.iter().filter(|c| c.is_selected).cloned().collect();
            let rank = poker::get_hand_rank(&selected, stats);
            let (base_chips, base_mult) = poker::get_hand_base_score(rank);

            stats.chips += base_chips;
            stats.mult += base_mult;

            // NEW: Apply Relic (Joker) Bonuses before final calc
            poker::apply_relic_bonuses(stats, &selected);

            let final_score = stats.chips * stats.mult;
            stats.total_score += final_score;
            stats.round_score += final_score;

            // Trigger HP Damage Flash
            stats.display_score += final_score as f32;
            stats.damage_flash_timer = 0.25;

            // Remove Cards & Decrement Hands
            hand.retain(|c| !c.is_selected);
            stats.hands_remaining -= 1;

            // CHECK FOR WIN/LOSS
            if stats.round_score >= stats.target_score {
                stats.window_y_offset = SCREEN_HEIGHT;
                *state = GameState::Shop;
                // Award money or other bonuses here
                stats.money += 10; // Placeholder
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
                    new_card.current_pos = Vector2::new(DECK_X, DECK_Y); // Appears at deck
                    hand.push(new_card);
                } else { break; }
            }

            // Auto Sort
            sort_hand(hand, stats.current_sort);

            // Reset Calculation Stats
            stats.chips = 0;
            stats.mult = 0;
            if stats.equipped_runes.iter().any(|r| r.name == "Force") { stats.mult = 10; }
            if stats.equipped_runes.iter().any(|r| r.name == "Flow") { stats.chips = 10; }
            if stats.equipped_runes.iter().any(|r| r.name == "Wealth") { stats.money += 3; }

            *animation_state = AnimationState::Idle;
        }
    }

    // --- PHYSICS & PREVIEW ---
    calculate_hand_positions(hand, animation_state);
    for card in hand.iter_mut() {
        card.update_anim(dt, total_time);
    }

    if *animation_state == AnimationState::Idle {
        let selected_cards: Vec<Card> = hand.iter().filter(|c| c.is_selected).cloned().collect();
        if !selected_cards.is_empty() {
            let rank = poker::get_hand_rank(&selected_cards, stats);
            let (chips, mult) = poker::get_hand_base_score(rank);
            stats.chips = chips;
            stats.mult = mult;
            // Note: We don't apply relic bonuses here in preview to avoid complex clone/calc overhead,
            // but you could add a 'preview_relic_bonuses' later if desired.
        } else {
            stats.chips = 0;
            stats.mult = 0;
        }
    }
}

fn sort_hand(hand: &mut Vec<Card>, mode: SortMode) {
    match mode {
        SortMode::Rank => hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit))),
        SortMode::Suit => hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value))),
    }
}

fn calculate_hand_positions(hand: &mut Vec<Card>, anim_state: &AnimationState) {
    let count = hand.len();
    if count == 0 { return; }
    let center_x = SCREEN_WIDTH / 2.0;
    let card_spacing = 100.0;
    let total_w = (count as f32 - 1.0) * card_spacing;
    let start_x = center_x - total_w / 2.0;

    let animation_duration = 0.2;

    for (i, card) in hand.iter_mut().enumerate() {
        // LOCK: Do not move selected cards if animation is running
        if *anim_state != AnimationState::Idle && card.is_selected {
            continue;
        }

        if card.is_dragging { continue; }

        let target_x = start_x + (i as f32 * card_spacing);
        let target_y = if card.is_selected {
            HAND_Y_POS - 40.0
        } else {
            HAND_Y_POS
        };
        card.set_target_pos_tweened(Vector2::new(target_x, target_y), animation_duration);
    }
}
