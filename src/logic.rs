use crate::poker;
use raylib::prelude::*;
use crate::structures::stats::{BaseModifiers, FloatingText, SortMode};
use crate::structures::card::Card;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;

// --- GAMEPLAY LOGIC ---
pub fn update_game(rl: &RaylibHandle, hand: &mut Vec<Card>, deck: &mut Vec<Card>, stats: &mut BaseModifiers, dt: f32, state: &mut GameState, animation_state: &mut AnimationState, total_time: f32) {
    let mouse_pos = rl.get_mouse_position();
    let mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_released = rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);
    let tab_held = rl.is_key_down(KeyboardKey::KEY_TAB);

    // --- UPDATE FLOATING TEXTS ---
    stats.floating_texts.retain(|ft| ft.life > 0.0);
    for ft in stats.floating_texts.iter_mut() {
        ft.pos += ft.vel * dt;
        ft.life -= dt;
        ft.vel.y -= 100.0 * dt;
    }

    // --- DEV TOOLBOX ---
    if tab_held && mouse_pressed {
        let start_x = DEV_BOX_X + 10.0;
        let mut start_y = DEV_BOX_Y + 40.0;

        let mut check_dev_btn = |_label: &str| -> bool {
            let rect = Rectangle::new(start_x, start_y, DEV_BTN_W, DEV_BTN_H);
            start_y += DEV_BTN_H + DEV_GAP;
            rect.check_collision_point_rec(mouse_pos)
        };

        if check_dev_btn("+ Discard") { stats.discards_remaining += 1; }
        if check_dev_btn("+ Hand") { stats.hands_remaining += 1; }
        if check_dev_btn("+ XP (Test)") {
            stats.xp += 50;
            if stats.xp >= stats.xp_target {
                stats.xp -= stats.xp_target;
                stats.level += 1;
                stats.xp_target += 50;
                stats.stat_points += 3;
            }
        }
        if check_dev_btn("+ Money") { stats.money += 10; }
        if check_dev_btn("Insta-Win") { stats.total_score += 10000; }
    }

    // --- INTERACTION ---
    if !tab_held && *animation_state == AnimationState::Idle {
        let mut hovered_index = None;
        let mut interaction_index = None;

        if let Some(idx) = hand.iter().position(|c| c.is_pressed || c.is_dragging) {
            interaction_index = Some(idx);
            hovered_index = Some(idx);
        } else {
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

        if mouse_pressed {
            if let Some(idx) = hovered_index {
                let card = &mut hand[idx];
                card.is_pressed = true;
                card.is_dragging = false;
                card.click_pos = mouse_pos;
            }
        }

        if mouse_down {
            if let Some(idx) = interaction_index {
                let card = &mut hand[idx];
                if card.is_pressed && !card.is_dragging {
                    let distance = ((mouse_pos.x - card.click_pos.x).powi(2) + (mouse_pos.y - card.click_pos.y).powi(2)).sqrt();
                    if distance > 5.0 {
                        card.is_dragging = true;
                        card.scale = SELECTED_SCALE * 1.1;
                    }
                }
                if card.is_dragging {
                    card.target_pos.x = mouse_pos.x;
                    if card.is_selected { card.target_pos.y = HAND_Y_POS - 60.0; }
                    else { card.target_pos.y = HAND_Y_POS; }

                    if idx > 0 && hand[idx].target_pos.x < hand[idx - 1].target_pos.x { hand.swap(idx, idx - 1); }
                    if idx < hand.len() - 1 && hand[idx].target_pos.x > hand[idx + 1].target_pos.x { hand.swap(idx, idx + 1); }
                }
            }
        }

        if mouse_released {
            if let Some(idx) = interaction_index {
                let selected_count = hand.iter().filter(|c| c.is_selected).count();
                let card = &mut hand[idx];
                if card.is_dragging {
                    card.is_dragging = false;
                    card.is_pressed = false;
                } else if card.is_pressed {
                    card.is_pressed = false;
                    if card.is_selected { card.is_selected = false; }
                    else if selected_count < 5 { card.is_selected = true; }
                }
            }
        }

        for (i, card) in hand.iter_mut().enumerate() {
            let is_hovered = Some(i) == hovered_index;
            if card.is_hovered != is_hovered { card.is_hovered = is_hovered; }
        }
    }

    calculate_hand_positions(hand, animation_state);
    for card in hand.iter_mut() { card.update_anim(dt, total_time); }

    // --- SCORING PREVIEW ---
    if *animation_state == AnimationState::Idle {
        let selected_cards: Vec<Card> = hand.iter().filter(|c| c.is_selected).cloned().collect();
        if !selected_cards.is_empty() {
            let rank = poker::get_hand_rank(&selected_cards);
            stats.hand_rank = Some(rank);
            let (base_chips, base_mult) = poker::get_hand_base_score(rank);
            let level_bonus = (stats.level - 1) * 5;

            let mut card_chips = 0;
            for card in &selected_cards {
                card_chips += poker::get_card_chip_value(card);
            }
            stats.chips = base_chips + level_bonus + card_chips;
            stats.mult = base_mult + (stats.level - 1);
        } else {
            stats.hand_rank = None;
            stats.chips = 0;
            stats.mult = 0;
        }
    }

    // --- BUTTONS ---
    let center_x = SIDEBAR_WIDTH + (SCREEN_WIDTH - SIDEBAR_WIDTH)/2.0;
    let btn_y = 660.0;
    let sort_y = 620.0;
    let play_btn = Rectangle::new(center_x - BTN_WIDTH - 10.0, btn_y, BTN_WIDTH, BTN_HEIGHT);
    let discard_btn = Rectangle::new(center_x + 10.0, btn_y, BTN_WIDTH, BTN_HEIGHT);
    let sort_rank_btn = Rectangle::new(center_x - SORT_BTN_WIDTH - 5.0, sort_y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let sort_suit_btn = Rectangle::new(center_x + 5.0, sort_y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let stats_btn = Rectangle::new(STATS_BTN_X, STATS_BTN_Y, STATS_BTN_W, STATS_BTN_H);

    if !tab_held && *animation_state == AnimationState::Idle && mouse_pressed {
        let selected_cards_count = hand.iter().filter(|c| c.is_selected).count();
        if play_btn.check_collision_point_rec(mouse_pos) && stats.hands_remaining > 0 && selected_cards_count > 0 {
            *animation_state = AnimationState::Playing;
        }
        if discard_btn.check_collision_point_rec(mouse_pos) && stats.discards_remaining > 0 && selected_cards_count > 0 {
            hand.retain(|c| !c.is_selected);
            let cards_needed = (stats.hand_size as usize).saturating_sub(hand.len());
            if cards_needed > 0 {
                stats.discards_remaining -= 1;
                // Pass current_sort preference
                draw_cards(hand, deck, cards_needed, stats.current_sort);
            }
        }

        // UPDATE: Set Sort Mode and Sort immediately
        if sort_rank_btn.check_collision_point_rec(mouse_pos) {
            stats.current_sort = SortMode::Rank;
            hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit)));
        }
        if sort_suit_btn.check_collision_point_rec(mouse_pos) {
            stats.current_sort = SortMode::Suit;
            hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value)));
        }

        if stats_btn.check_collision_point_rec(mouse_pos) {
            *state = GameState::StatsMenu;
        }
    }

    // --- PHASE 1: PLAY ANIMATION ---
    if *animation_state == AnimationState::Playing {
        let mut all_arrived = true;
        let played_count = hand.iter().filter(|c| c.is_selected).count();
        let played_spacing = 150.0;
        let start_x = center_x - ((played_count as f32 - 1.0) * played_spacing) / 2.0;

        let mut idx = 0;
        for card in hand.iter_mut() {
            if card.is_selected {
                let target_x = start_x + (idx as f32 * played_spacing);
                card.target_pos = Vector2::new(target_x, PLAYED_Y_POS);
                card.target_scale = PLAYED_SCALE;
                card.target_rotation = 0.0;
                if (card.current_pos.y - PLAYED_Y_POS).abs() > 2.0 { all_arrived = false; }
                idx += 1;
            }
        }

        if all_arrived {
            stats.hands_remaining -= 1;

            let roll = unsafe { raylib::ffi::GetRandomValue(0, 100) } as f32 / 100.0;
            stats.is_crit_active = roll < stats.crit_chance;

            if let Some(rank) = stats.hand_rank {
                let (base_chips, base_mult) = poker::get_hand_base_score(rank);
                let level_bonus = (stats.level - 1) * 5;
                stats.chips = base_chips + level_bonus;
                stats.mult = base_mult + (stats.level - 1);
            }

            stats.score_index = 0;
            stats.score_timer = 0.5;
            stats.score_delay = 0.5;

            *animation_state = AnimationState::ScoringSeq;
        }
    }

    // --- PHASE 2: SCORING SEQUENCE ---
    if *animation_state == AnimationState::ScoringSeq {
        stats.score_timer -= dt;

        if stats.score_timer <= 0.0 {
            let mut current_sel_idx = 0;
            let mut found_card_idx = None;

            for (i, card) in hand.iter().enumerate() {
                if card.is_selected {
                    if current_sel_idx == stats.score_index {
                        found_card_idx = Some(i);
                        break;
                    }
                    current_sel_idx += 1;
                }
            }

            if let Some(idx) = found_card_idx {
                let card = &mut hand[idx];
                let val = poker::get_card_chip_value(card);
                stats.chips += val;

                let pop_size = if stats.is_crit_active { PLAYED_SCALE * 1.6 } else { PLAYED_SCALE * 1.35 };
                card.scale = pop_size;

                let (txt_size, txt_col) = if stats.is_crit_active { (50, NEU_YELLOW) } else { (40, NEU_BLUE) };

                stats.floating_texts.push(FloatingText {
                    pos: Vector2::new(card.current_pos.x - 20.0, card.current_pos.y - 100.0),
                    vel: Vector2::new(0.0, -150.0),
                    text: format!("+{}", val),
                    color: txt_col,
                    size: txt_size,
                    life: 0.8,
                    max_life: 0.8,
                });

                stats.score_index += 1;
                let speed_factor = if stats.is_crit_active { 0.4 } else { 0.6 };
                stats.score_delay *= speed_factor;
                stats.score_timer = stats.score_delay.max(0.05);
            } else {
                let raw_score = stats.chips * stats.mult;

                if stats.is_crit_active {
                    stats.total_score += (raw_score as f32 * stats.crit_mult) as i32;
                    stats.floating_texts.push(FloatingText {
                        pos: Vector2::new(center_x - 120.0, PLAYED_Y_POS - 180.0),
                        vel: Vector2::new(0.0, -50.0),
                        text: "CRITICAL!".to_string(),
                        color: NEU_YELLOW,
                        size: 70,
                        life: 2.0,
                        max_life: 2.0,
                    });
                } else {
                    stats.total_score += raw_score;
                }

                stats.chips = 0;
                stats.mult = 0;
                stats.hand_rank = None;
                *animation_state = AnimationState::Scoring;
            }
        }
    }

    // --- PHASE 3: CLEANUP ---
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
                stats.is_crit_active = false;

                if stats.total_score >= stats.target_score {
                    if stats.xp >= stats.xp_target {
                        stats.xp -= stats.xp_target;
                        stats.level += 1;
                        stats.xp_target += 50;
                        stats.stat_points += 3;
                        *state = GameState::StatsMenu;
                    } else {
                        *state = GameState::Shop;
                    }
                }
                else if stats.hands_remaining == 0 {
                    stats.current_hp -= stats.enemy_damage;
                    if stats.current_hp <= 0 {
                        *state = GameState::GameOver;
                    } else {
                        stats.money += 2;
                        *state = GameState::Shop;
                    }
                } else {
                    let cards_needed = (stats.hand_size as usize).saturating_sub(hand.len());
                    // Pass current_sort preference
                    draw_cards(hand, deck, cards_needed, stats.current_sort);
                }
            }
        } else {
            let speed = (diff * 10.0 * dt).max(50.0 * dt);
            stats.display_score += speed;
        }
    }

    stats.deck_count = deck.len() as i32;
}

// ... [calculate_hand_positions, update_menu, update_shop, update_stats_menu] ...
fn calculate_hand_positions(hand: &mut Vec<Card>, anim_state: &AnimationState) {
    let count = hand.len();
    if count == 0 { return; }
    let center_x = SIDEBAR_WIDTH + (SCREEN_WIDTH - SIDEBAR_WIDTH) / 2.0;
    let hand_y = HAND_Y_POS;
    let card_spacing = 100.0;
    let hand_start_x = center_x - ((count as f32 - 1.0) * card_spacing) / 2.0;

    for (i, card) in hand.iter_mut().enumerate() {
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
        if *anim_state != AnimationState::Idle && card.is_selected { continue; }

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

pub fn update_shop(rl: &RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers, hand: &mut Vec<Card>, deck: &mut Vec<Card>) {
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let btn_rect = Rectangle::new(SCREEN_WIDTH/2.0 - 100.0, SCREEN_HEIGHT/2.0 + 100.0, 200.0, 60.0);
    let stats_btn = Rectangle::new(STATS_BTN_X, STATS_BTN_Y, STATS_BTN_W, STATS_BTN_H);

    if clicked {
        if btn_rect.check_collision_point_rec(mouse_pos) {
            stats.total_score = 0;
            stats.display_score = 0.0;
            stats.hands_remaining = 4;
            stats.discards_remaining = 3;
            stats.chips = 0;
            stats.mult = 0;
            hand.clear();
            deck.clear();
            let mut id_counter = 0;
            for suit in 0..4 {
                for val in 2..15 {
                    let mut card = Card::new(id_counter, DECK_X, DECK_Y);
                    card.suit = suit;
                    card.value = val;
                    deck.push(card);
                    id_counter += 1;
                }
            }
            for i in 0..deck.len() {
                let swap_idx = unsafe { raylib::ffi::GetRandomValue(0, 51) } as usize;
                deck.swap(i, swap_idx);
            }
            for _ in 0..stats.hand_size {
                if let Some(mut card) = deck.pop() {
                    card.current_pos = Vector2::new(DECK_X, DECK_Y);
                    card.target_pos.y = HAND_Y_POS;
                    hand.push(card);
                }
            }
            // Auto sort on new round based on preference
            match stats.current_sort {
                SortMode::Rank => hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit))),
                SortMode::Suit => hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value))),
            }

            *state = GameState::Playing;
        }
        if stats_btn.check_collision_point_rec(mouse_pos) {
            *state = GameState::StatsMenu;
        }
    }
}

pub fn update_stats_menu(rl: &RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers) {
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let center_x = SCREEN_WIDTH/2.0 - STAT_WIN_W/2.0;
    let center_y = SCREEN_HEIGHT/2.0 - STAT_WIN_H/2.0;
    let back_btn = Rectangle::new(center_x + 150.0, center_y + STAT_WIN_H - 60.0, 200.0, 40.0);

    if clicked {
        if back_btn.check_collision_point_rec(mouse_pos) {
            *state = GameState::Playing;
        }
        if stats.stat_points > 0 {
            let y_hp = center_y + 100.0;
            let btn_hp = Rectangle::new(center_x + 350.0, y_hp, 30.0, 30.0);
            if btn_hp.check_collision_point_rec(mouse_pos) {
                stats.stat_points -= 1;
                stats.max_hp += 5;
                stats.current_hp += 5;
            }
            let y_crit = center_y + 150.0;
            let btn_crit = Rectangle::new(center_x + 350.0, y_crit, 30.0, 30.0);
            if btn_crit.check_collision_point_rec(mouse_pos) {
                stats.stat_points -= 1;
                stats.crit_chance += 0.05;
            }
            let y_mult = center_y + 200.0;
            let btn_mult = Rectangle::new(center_x + 350.0, y_mult, 30.0, 30.0);
            if btn_mult.check_collision_point_rec(mouse_pos) {
                stats.stat_points -= 1;
                stats.crit_mult += 0.5;
            }
        }
    }
}

// UPDATE: Added sort_mode param and sorting logic
fn draw_cards(hand: &mut Vec<Card>, deck: &mut Vec<Card>, count: usize, sort_mode: SortMode) {
    for _ in 0..count {
        if let Some(mut card) = deck.pop() {
            card.current_pos = Vector2::new(DECK_X, DECK_Y);
            card.rotation = 1.0;
            hand.push(card);
        }
    }
    // Auto sort new cards
    match sort_mode {
        SortMode::Rank => hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit))),
        SortMode::Suit => hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value))),
    }
}