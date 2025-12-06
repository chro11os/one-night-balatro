use crate::poker;
use raylib::prelude::*;
use crate::structures::stats::{BaseModifiers, BossAbility, SortMode}; // FIX: Added SortMode
use crate::structures::card::Card;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;

pub fn update_game(rl: &RaylibHandle, hand: &mut Vec<Card>, _deck: &mut Vec<Card>, stats: &mut BaseModifiers, dt: f32, _state: &mut GameState, animation_state: &mut AnimationState, total_time: f32) {
    let mouse_pos = rl.get_mouse_position();
    let mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_released = rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);

    let play_btn = Rectangle::new(PLAY_BTN_POS.x, PLAY_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);
    let discard_btn = Rectangle::new(DISC_BTN_POS.x, DISC_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);

    let sort_rank_btn = Rectangle::new(SORT_RANK_POS.x, SORT_RANK_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let sort_suit_btn = Rectangle::new(SORT_SUIT_POS.x, SORT_SUIT_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);

    if *animation_state == AnimationState::Idle {
        let mut hovered_index = None;

        for (i, card) in hand.iter_mut().enumerate().rev() {
            let width = CARD_WIDTH * card.target_scale;
            let height = CARD_HEIGHT * card.target_scale;
            let rect = Rectangle::new(card.current_pos.x - width/2.0, card.current_pos.y - height/2.0, width, height);

            if rect.check_collision_point_rec(mouse_pos) {
                hovered_index = Some(i);
                break;
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
                let card = &mut hand[idx];
                card.target_pos = mouse_pos;
                card.is_dragging = true;
            }
        }

        if mouse_released {
            for card in hand.iter_mut() {
                if card.is_pressed && !card.is_dragging {
                    card.is_selected = !card.is_selected;
                }
                card.is_pressed = false;
                card.is_dragging = false;
            }
        }

        if mouse_pressed {
            let selected_count = hand.iter().filter(|c| c.is_selected).count();

            if play_btn.check_collision_point_rec(mouse_pos) && stats.hands_remaining > 0 && selected_count > 0 {
                *animation_state = AnimationState::Playing;
            }
            if discard_btn.check_collision_point_rec(mouse_pos) && stats.discards_remaining > 0 && selected_count > 0 {
                stats.discards_remaining -= 1;
                hand.retain(|c| !c.is_selected);
            }

            // --- SORT LOGIC ---
            if sort_rank_btn.check_collision_point_rec(mouse_pos) {
                stats.current_sort = SortMode::Rank;
                sort_hand(hand, SortMode::Rank);
            }
            if sort_suit_btn.check_collision_point_rec(mouse_pos) {
                stats.current_sort = SortMode::Suit;
                sort_hand(hand, SortMode::Suit);
            }
        }
    }

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
        } else {
            stats.chips = 0;
            stats.mult = 0;
        }
    }
}

fn sort_hand(hand: &mut Vec<Card>, mode: SortMode) {
    match mode {
        SortMode::Rank => {
            hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit)));
        }
        SortMode::Suit => {
            hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value)));
        }
    }
}

fn calculate_hand_positions(hand: &mut Vec<Card>, _anim_state: &AnimationState) {
    let count = hand.len();
    if count == 0 { return; }
    let center_x = SCREEN_WIDTH / 2.0;
    let card_spacing = 100.0;
    let total_w = (count as f32 - 1.0) * card_spacing;
    let start_x = center_x - total_w / 2.0;

    for (i, card) in hand.iter_mut().enumerate() {
        if card.is_dragging { continue; }
        card.target_pos.x = start_x + (i as f32 * card_spacing);
        if card.is_selected { card.target_pos.y = HAND_Y_POS - 40.0; } else { card.target_pos.y = HAND_Y_POS; }
    }
}

// FIX: Only one definition of update_menu now
pub fn update_menu(rl: &RaylibHandle, state: &mut GameState) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        *state = GameState::RuneSelect;
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
            let start_x = (center_x + content_offset) - ((count as f32 - 1.0) * spacing) / 2.0;

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

            for rune in &stats.equipped_runes {
                match rune.name.as_str() {
                    "Paladin" => {
                        stats.max_hp += 40;
                        stats.current_hp += 40;
                    },
                    "Reaper" => {
                        stats.max_hp -= 15;
                        if stats.max_hp < 1 { stats.max_hp = 1; }
                        stats.current_hp = stats.max_hp;
                    },
                    "Judgement" => {
                        stats.ante_scaling = 2.0; // "Hard Mode" scaling
                        // Note: "Balanced Calc" logic needs to be in poker.rs scoring
                    },
                    "Greed" => {
                        stats.hands_remaining += 1;
                        stats.discards_remaining += 1;
                    },
                    "Investment" => {
                        stats.money = 0;
                    },
                    "Merchant" => stats.shop_price_mult = 1.2,
                    "Evolution" => {
                        stats.ante_scaling = 2.25; // 1.5 * 1.5 approx
                    },
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

pub fn update_shop(rl: &RaylibHandle, state: &mut GameState, _stats: &mut BaseModifiers, _hand: &mut Vec<Card>, _deck: &mut Vec<Card>) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        *state = GameState::Playing;
    }
}
pub fn update_stats_menu(_rl: &RaylibHandle, _state: &mut GameState, _stats: &mut BaseModifiers) {}
pub fn update_battle_result(_rl: &RaylibHandle, _state: &mut GameState, _stats: &mut BaseModifiers) {}