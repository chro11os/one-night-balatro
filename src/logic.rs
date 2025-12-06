use crate::poker;
use raylib::prelude::*;
use crate::structures::stats::{BaseModifiers, BossAbility, SortMode};
use crate::structures::card::Card;
use crate::structures::state::{GameState, AnimationState};

pub fn update_game(rl: &RaylibHandle, hand: &mut Vec<Card>, deck: &mut Vec<Card>, stats: &mut BaseModifiers, dt: f32, state: &mut GameState, animation_state: &mut AnimationState, total_time: f32) {
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
                let width = CARD_WIDTH * card.target_scale;
                let height = CARD_HEIGHT * card.target_scale;
                let rect = Rectangle::new(card.current_pos.x - width / 2.0, card.current_pos.y - height / 2.0, width, height);

                if rect.check_collision_point_rec(mouse_pos) {
                    hovered_index = Some(i);
                    break;
                }
            }

            for (i, card) in hand.iter_mut().enumerate() {
                card.is_hovered = Some(i) == hovered_index;
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
                    hand.retain(|c| !c.is_selected);
                    while hand.len() < stats.hand_size as usize {
                        if let Some(mut new_card) = deck.pop() {
                            new_card.current_pos = Vector2::new(DECK_X, DECK_Y);
                            new_card.target_pos = Vector2::new(DECK_X, DECK_Y);
                            hand.push(new_card);
                        } else { break; }
                    }
                    sort_hand(hand, stats.current_sort);
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

        AnimationState::Playing => {
            // PHASE 1: Move Cards to Center
            let center_x = SCREEN_WIDTH / 2.0;
            let selected_count = hand.iter().filter(|c| c.is_selected).count();
            let spacing = 120.0;
            let start_x = center_x - ((selected_count as f32 - 1.0) * spacing) / 2.0;

            let mut idx = 0;
            for card in hand.iter_mut() {
                if card.is_selected {
                    card.target_pos.x = start_x + (idx as f32 * spacing);
                    card.target_pos.y = PLAY_AREA_Y;
                    card.target_scale = 1.1;
                    idx += 1;
                }
            }

            let mut all_arrived = true;
            for card in hand.iter() {
                if card.is_selected {
                    if (card.current_pos - card.target_pos).length() > 10.0 {
                        all_arrived = false;
                        break;
                    }
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
                    hand[idx].scale = 1.6;

                    // Add Chips Logic
                    let chips = poker::get_card_chip_value(&hand[idx]);
                    stats.chips += chips;

                    // Spawn VFX
                    let pos = hand[idx].current_pos;
                    stats.spawn_floating_text(format!("+{}", chips), pos - Vector2::new(0.0, 90.0), NEU_BLUE);
                    stats.spawn_particle_burst(pos, NEU_YELLOW);

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
            let rank = poker::get_hand_rank(&selected);
            let (base_chips, base_mult) = poker::get_hand_base_score(rank);

            stats.chips += base_chips;
            stats.mult += base_mult;

            // NEW: Apply Relic (Joker) Bonuses before final calc
            poker::apply_relic_bonuses(stats, &selected);

            let final_score = stats.chips * stats.mult;
            stats.total_score += final_score;

            // Trigger HP Damage Flash
            stats.display_score += final_score as f32;
            stats.damage_flash_timer = 0.25;

            // Remove Cards & Decrement Hands
            hand.retain(|c| !c.is_selected);
            stats.hands_remaining -= 1;

            // Refill Hand
            while hand.len() < stats.hand_size as usize {
                if let Some(mut new_card) = deck.pop() {
                    new_card.current_pos = Vector2::new(DECK_X, DECK_Y);
                    new_card.target_pos = Vector2::new(DECK_X, DECK_Y);
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
            let rank = poker::get_hand_rank(&selected_cards);
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

use crate::consts::*;

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

    for (i, card) in hand.iter_mut().enumerate() {
        // LOCK: Do not move selected cards if animation is running
        if *anim_state != AnimationState::Idle && card.is_selected {
            continue;
        }

        if card.is_dragging { continue; }

        card.target_pos.x = start_x + (i as f32 * card_spacing);

        if card.is_selected {
            card.target_pos.y = HAND_Y_POS - 40.0;
        } else {
            card.target_pos.y = HAND_Y_POS;
        }
    }
}

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

pub fn update_menu(rl: &RaylibHandle, state: &mut GameState) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        *state = GameState::RuneSelect;
    }
}
pub fn update_shop(rl: &RaylibHandle, state: &mut GameState, _stats: &mut BaseModifiers, _hand: &mut Vec<Card>, _deck: &mut Vec<Card>) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        *state = GameState::Playing;
    }
}
pub fn update_battle_result(_rl: &RaylibHandle, _state: &mut GameState, _stats: &mut BaseModifiers) {}