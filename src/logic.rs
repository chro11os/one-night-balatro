use crate::poker;
use raylib::prelude::*;
use crate::structures::stats::{BaseModifiers, FloatingText, Particle, SortMode, BossAbility, RuneType};
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

    let center_x = (SIDEBAR_WIDTH + (SCREEN_WIDTH - SIDEBAR_WIDTH) / 2.0) + CENTER_OFFSET_X;

    // ... (Floating Text, Particles, Dev Toolbox, Interaction logic omitted - keep as is) ...
    // Note: Pasting just the button logic update for brevity, assuming standard logic.rs structure

    // --- BUTTONS ---
    // Action Buttons (Play/Discard)
    let play_btn = Rectangle::new(center_x - BTN_WIDTH - 10.0, ACTION_BTN_Y, BTN_WIDTH, BTN_HEIGHT);
    let discard_btn = Rectangle::new(center_x + 10.0, ACTION_BTN_Y, BTN_WIDTH, BTN_HEIGHT);

    // Sort Buttons (Below Deck)
    let sort_rank_btn = Rectangle::new(SORT_RANK_POS.x, SORT_RANK_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let sort_suit_btn = Rectangle::new(SORT_SUIT_POS.x, SORT_SUIT_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);

    let info_btn_rect = Rectangle::new(90.0, 650.0, 120.0, 40.0);

    // ... (Hover Logic etc) ...

    if !tab_held && *animation_state == AnimationState::Idle && mouse_pressed {
        let selected_cards_count = hand.iter().filter(|c| c.is_selected).count();

        // Play Input
        if play_btn.check_collision_point_rec(mouse_pos) && stats.hands_remaining > 0 && selected_cards_count > 0 {
            *animation_state = AnimationState::Playing;
        }

        // Discard Input
        if discard_btn.check_collision_point_rec(mouse_pos) && stats.discards_remaining > 0 && selected_cards_count > 0 {
            let mut can_discard = true;
            if stats.active_ability == BossAbility::PayToDiscard {
                if stats.money >= 1 { stats.money -= 1; } else { can_discard = false; }
            }
            if can_discard {
                hand.retain(|c| !c.is_selected);
                let effective_hand_size = if stats.active_ability == BossAbility::HandSizeMinusOne { stats.hand_size - 1 } else { stats.hand_size };
                let cards_needed = (effective_hand_size as usize).saturating_sub(hand.len());
                if cards_needed > 0 {
                    stats.discards_remaining -= 1;
                    draw_cards(hand, deck, cards_needed, stats.current_sort);
                }
            }
        }

        // Sorting
        if sort_rank_btn.check_collision_point_rec(mouse_pos) {
            stats.current_sort = SortMode::Rank;
            hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit)));
        }
        if sort_suit_btn.check_collision_point_rec(mouse_pos) {
            stats.current_sort = SortMode::Suit;
            hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value)));
        }

        if info_btn_rect.check_collision_point_rec(mouse_pos) {
            stats.previous_state = *state;
            *state = GameState::StatsMenu;
        }
    }

    // --- PHASE 1: PLAY ANIMATION ---
    if *animation_state == AnimationState::Playing {
        let mut all_arrived = true;
        let played_count = hand.iter().filter(|c| c.is_selected).count();
        // Smaller spacing for the "Depth" look
        let played_spacing = 110.0;
        let start_x = center_x - ((played_count as f32 - 1.0) * played_spacing) / 2.0;

        let selected_cards: Vec<Card> = hand.iter().filter(|c| c.is_selected).cloned().collect();
        let scoring_ids = poker::get_scoring_ids(&selected_cards);

        let mut idx = 0;
        for card in hand.iter_mut() {
            if card.is_selected {
                let target_x = start_x + (idx as f32 * played_spacing);
                card.target_pos = Vector2::new(target_x, PLAYED_Y_POS);

                if scoring_ids.contains(&card.id) {
                    card.target_scale = PLAYED_SCALE; // Use the smaller depth scale
                }
                else {
                    card.target_scale = JUNK_SCALE * 0.8;
                }

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

                if stats.equipped_runes.iter().any(|r| r.name == "Flow") { stats.chips += 10; }
                if stats.equipped_runes.iter().any(|r| r.name == "Force") { stats.mult += 10; }
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
            let selected_cards: Vec<Card> = hand.iter().filter(|c| c.is_selected).cloned().collect();
            let scoring_ids = poker::get_scoring_ids(&selected_cards);

            for (i, card) in hand.iter().enumerate() {
                if card.is_selected {
                    if current_sel_idx == stats.score_index { found_card_idx = Some(i); break; }
                    current_sel_idx += 1;
                }
            }

            if let Some(idx) = found_card_idx {
                let card = &mut hand[idx];
                if scoring_ids.contains(&card.id) {
                    let mut is_debuffed = false;
                    if let BossAbility::SilenceSuit(s) = stats.active_ability {
                        if card.suit == s { is_debuffed = true; }
                    }

                    if is_debuffed {
                        stats.floating_texts.push(FloatingText {
                            pos: Vector2::new(card.current_pos.x - 30.0, card.current_pos.y - 50.0),
                            vel: Vector2::new(0.0, -50.0),
                            text: "DEBUFF".to_string(),
                            color: NEU_BLACK,
                            size: 30,
                            life: 1.0,
                            max_life: 1.0,
                        });
                        card.scale = PLAYED_SCALE * 0.9;
                    } else {
                        let val = poker::get_card_chip_value(card);
                        stats.chips += val;

                        // [ARTIFACT HOOK] - OnCardScore
                        // e.g. if card.suit == Suit::Hearts && stats.has_artifact("Bloodstone") { ... }

                        // IMPACT VISUALS
                        card.scale = 2.0;

                        let (txt_size, txt_col) = if stats.is_crit_active { (60, NEU_YELLOW) } else { (45, NEU_BLUE) };
                        let vx = unsafe { raylib::ffi::GetRandomValue(-50, 50) } as f32;

                        stats.floating_texts.push(FloatingText {
                            pos: Vector2::new(card.current_pos.x - 20.0, card.current_pos.y - 100.0),
                            vel: Vector2::new(vx, -250.0),
                            text: format!("+{}", val),
                            color: txt_col,
                            size: txt_size,
                            life: 0.6,
                            max_life: 0.6,
                        });

                        // Particle Effects...
                        let p_count = if stats.is_crit_active { 40 } else { 20 };
                        for _ in 0..p_count {
                            let vx = unsafe { raylib::ffi::GetRandomValue(-300, 300) } as f32;
                            let vy = unsafe { raylib::ffi::GetRandomValue(-400, 100) } as f32;
                            let size = unsafe { raylib::ffi::GetRandomValue(6, 14) } as f32;
                            let life = unsafe { raylib::ffi::GetRandomValue(5, 10) } as f32 / 10.0;
                            stats.particles.push(Particle {
                                pos: card.current_pos,
                                vel: Vector2::new(vx, vy),
                                color: if stats.is_crit_active { NEU_YELLOW } else { NEU_BLUE },
                                size,
                                life,
                                max_life: life,
                                rotation: 0.0,
                                rot_speed: vx * 0.1,
                            });
                        }
                    }
                    stats.score_timer = if stats.is_crit_active { 0.15 } else { 0.25 };
                } else {
                    stats.score_timer = 0.05;
                }
                stats.score_index += 1;
            } else {
                // [ARTIFACT HOOK] - OnHandEnd (Held in Hand effects?)

                let raw_score = stats.chips * stats.mult;
                if stats.is_crit_active {
                    stats.total_score += (raw_score as f32 * stats.crit_mult) as i32;
                    stats.shake_timer = 0.5;
                    stats.floating_texts.push(FloatingText {
                        pos: Vector2::new(center_x - 150.0, PLAYED_Y_POS - 200.0),
                        vel: Vector2::new(0.0, -20.0),
                        text: "CRITICAL!".to_string(),
                        color: NEU_YELLOW,
                        size: 80,
                        life: 1.5,
                        max_life: 1.5,
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
                    stats.round_won = true;
                    stats.xp += 50;
                    stats.money += 5;
                    *state = GameState::BattleResult;
                }
                else if stats.hands_remaining == 0 {
                    stats.round_won = false;
                    stats.current_hp -= stats.enemy_damage;
                    if stats.current_hp <= 0 {
                        *state = GameState::GameOver;
                    } else {
                        stats.money += 2;
                        *state = GameState::Shop;
                    }
                } else {
                    let effective_hand_size = if stats.active_ability == BossAbility::HandSizeMinusOne { stats.hand_size - 1 } else { stats.hand_size };
                    let cards_needed = (effective_hand_size as usize).saturating_sub(hand.len());
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
pub fn update_battle_result(rl: &RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers) {
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let center_x = SCREEN_WIDTH/2.0;
    let center_y = SCREEN_HEIGHT/2.0;
    let btn_rect = Rectangle::new(center_x - 100.0, center_y + 100.0, 200.0, 60.0);
    if clicked && btn_rect.check_collision_point_rec(mouse_pos) {
        if stats.xp >= stats.xp_target {
            stats.xp -= stats.xp_target;
            stats.level += 1;
            stats.xp_target += 50;
            stats.stat_points += 3;
            stats.previous_state = GameState::Shop;
            *state = GameState::StatsMenu;
        } else {
            *state = GameState::Shop;
        }
    }
}

pub fn update_rune_select(rl: &RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers) {
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    // LAYOUT RECALC (Must match draw_scene.rs)
    let panel_w = 320.0;
    let main_area_w = SCREEN_WIDTH - panel_w;
    let center_x = main_area_w / 2.0;

    let rows = [
        (RuneType::Red, 130.0),
        (RuneType::Blue, 250.0),
        (RuneType::Green, 370.0),
        (RuneType::Minor, 490.0),
    ];

    if clicked {
        // Hit Logic for Runes
        for (r_type, start_y) in rows.iter() {
            let row_runes: Vec<_> = stats.available_runes.iter().filter(|r| r.rune_type == *r_type).collect();
            let spacing = 150.0;
            let row_start_x = center_x - ((row_runes.len() as f32 - 1.0) * spacing) / 2.0;

            for (i, rune) in row_runes.iter().enumerate() {
                let x = row_start_x + (i as f32 * spacing);
                let y = *start_y + 30.0;
                let radius = 30.0;
                let dist = ((mouse_pos.x - x).powi(2) + (mouse_pos.y - y).powi(2)).sqrt();

                if dist < radius {
                    // EXCLUSIVITY LOGIC:
                    // Remove any existing rune of the SAME type
                    stats.equipped_runes.retain(|r| r.rune_type != *r_type);

                    // Add new one
                    stats.equipped_runes.push((*rune).clone());
                }
            }
        }

        // Start Button (Matches layout in draw_scene)
        let panel_x = main_area_w;
        let start_btn_w = panel_w - 60.0;
        let start_btn_h = 60.0;
        let start_btn_x = panel_x + 30.0;
        let start_btn_y = SCREEN_HEIGHT - 100.0;
        let start_btn = Rectangle::new(start_btn_x, start_btn_y, start_btn_w, start_btn_h);

        if start_btn.check_collision_point_rec(mouse_pos) {

            // RESET MODIFIERS BEFORE APPLYING
            stats.shop_price_mult = 1.0;
            stats.ante_scaling = 1.5;

            // APPLY PASSIVES
            for rune in &stats.equipped_runes {
                match rune.name.as_str() {
                    "Paladin" => {
                        stats.max_hp += 40;
                        stats.current_hp += 40;
                    }
                    "Reaper" => {
                        stats.max_hp -= 5;
                        stats.current_hp = stats.max_hp;
                    }
                    "Greed" => {
                        stats.hands_remaining += 1;
                        stats.discards_remaining += 1;
                    }
                    "Merchant" => {
                        stats.shop_price_mult = 1.2;
                    }
                    "Evolution" => {
                        stats.ante_scaling = 2.0; // Scales faster
                    }
                    // MINOR RUNES
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

fn calculate_hand_positions(hand: &mut Vec<Card>, anim_state: &AnimationState) {
    let count = hand.len();
    if count == 0 { return; }
    let center_x = (SIDEBAR_WIDTH + (SCREEN_WIDTH - SIDEBAR_WIDTH) / 2.0) + CENTER_OFFSET_X;
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
        *state = GameState::RuneSelect;
    }
}

pub fn update_shop(rl: &RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers, hand: &mut Vec<Card>, deck: &mut Vec<Card>) {
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let btn_rect = Rectangle::new(SCREEN_WIDTH/2.0 - 100.0, SCREEN_HEIGHT/2.0 + 100.0, 200.0, 60.0);
    let stats_btn = Rectangle::new(STATS_BTN_X, STATS_BTN_Y, STATS_BTN_W, STATS_BTN_H);

    if clicked {
        if btn_rect.check_collision_point_rec(mouse_pos) {
            // NEXT FIGHT SETUP
            if stats.round_won {
                stats.enemies_defeated += 1;

                // RUNE LOGIC
                if stats.equipped_runes.iter().any(|r| r.name == "Reaper") {
                    let heal = if stats.round % 8 == 0 { 2 } else { 1 };
                    stats.max_hp += heal;
                    stats.current_hp += heal;
                }
                if stats.equipped_runes.iter().any(|r| r.name == "Investment") {
                    let bonus = stats.enemies_defeated * 2;
                    stats.money += bonus;
                }
                if stats.equipped_runes.iter().any(|r| r.name == "Midas") {
                    let bonus = (stats.money as f32 * 0.25) as i32;
                    stats.money += bonus;
                }

                stats.round += 1;
                if stats.round > 8 {
                    stats.round = 1;
                    stats.ante += 1;
                }

                // SCALING LOGIC
                let mut diff_mult = stats.ante_scaling;
                if stats.equipped_runes.iter().any(|r| r.name == "Judgement") { diff_mult += 0.5; }
                stats.target_score = (stats.target_score as f32 * diff_mult) as i32;

                stats.enemy_damage += 2;
                let (name, ability) = get_new_enemy(stats.ante);
                stats.enemy_name = name;
                stats.active_ability = ability;
                if let BossAbility::DoubleTarget = stats.active_ability { stats.target_score *= 2; }
            } else {
                // LOSS Logic
                if stats.equipped_runes.iter().any(|r| r.name == "Midas") {
                    let penalty = (stats.money as f32 * 0.25) as i32;
                    stats.money -= penalty;
                }
            }

            stats.total_score = 0;
            stats.display_score = 0.0;
            stats.hands_remaining = 4;
            stats.discards_remaining = 3;
            stats.chips = 0;
            stats.mult = 0;

            // Re-apply Persistent Rune Buffs
            if stats.equipped_runes.iter().any(|r| r.name == "Greed") {
                stats.hands_remaining += 1;
                stats.discards_remaining += 1;
            }
            // Re-apply Minor Stat Runes (Base display)
            if stats.equipped_runes.iter().any(|r| r.name == "Force") { stats.mult = 10; }
            if stats.equipped_runes.iter().any(|r| r.name == "Flow") { stats.chips = 10; }

            reset_deck(hand, deck, stats);
            *state = GameState::Playing;
        }
        if stats_btn.check_collision_point_rec(mouse_pos) {
            stats.previous_state = GameState::Shop;
            *state = GameState::StatsMenu;
        }
    }
}

pub fn update_stats_menu(rl: &RaylibHandle, state: &mut GameState, stats: &mut BaseModifiers) {
    let mouse_pos = rl.get_mouse_position();
    let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let center_x = SCREEN_WIDTH/2.0 - STAT_WIN_W/2.0;
    let center_y = SCREEN_HEIGHT/2.0 - STAT_WIN_H/2.0;

    let back_btn = Rectangle::new(center_x + STAT_WIN_W/2.0 - 100.0, center_y + STAT_WIN_H - 60.0, 200.0, 40.0);

    if clicked {
        if back_btn.check_collision_point_rec(mouse_pos) {
            *state = stats.previous_state;
        }

        // Stat Upgrades (Coordinate Updates)
        if stats.stat_points > 0 {
            let col1_x = center_x + 40.0;
            let start_y = center_y + 80.0;
            let x_btn = col1_x + 220.0;

            let btn_hp = Rectangle::new(x_btn, start_y + 80.0, 30.0, 30.0);
            if btn_hp.check_collision_point_rec(mouse_pos) {
                stats.stat_points -= 1;
                stats.max_hp += 5;
                stats.current_hp += 5;
            }

            let btn_crit = Rectangle::new(x_btn, start_y + 130.0, 30.0, 30.0);
            if btn_crit.check_collision_point_rec(mouse_pos) {
                stats.stat_points -= 1;
                stats.crit_chance += 0.05;
            }

            let btn_mult = Rectangle::new(x_btn, start_y + 180.0, 30.0, 30.0);
            if btn_mult.check_collision_point_rec(mouse_pos) {
                stats.stat_points -= 1;
                stats.crit_mult += 0.5;
            }
        }
    }
}

fn reset_deck(hand: &mut Vec<Card>, deck: &mut Vec<Card>, stats: &mut BaseModifiers) {
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
    let start_draw_count = if stats.active_ability == BossAbility::HandSizeMinusOne { stats.hand_size - 1 } else { stats.hand_size };
    for _ in 0..start_draw_count {
        if let Some(mut card) = deck.pop() {
            card.current_pos = Vector2::new(DECK_X, DECK_Y);
            card.target_pos.y = HAND_Y_POS;
            hand.push(card);
        }
    }
    match stats.current_sort {
        SortMode::Rank => hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit))),
        SortMode::Suit => hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value))),
    }
}

fn draw_cards(hand: &mut Vec<Card>, deck: &mut Vec<Card>, count: usize, sort_mode: SortMode) {
    for _ in 0..count {
        if let Some(mut card) = deck.pop() {
            card.current_pos = Vector2::new(DECK_X, DECK_Y);
            card.rotation = 1.0;
            hand.push(card);
        }
    }
    match sort_mode {
        SortMode::Rank => hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit))),
        SortMode::Suit => hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value))),
    }
}

fn get_new_enemy(ante: i32) -> (String, BossAbility) {
    let tier_1 = ["Giant Rat", "Slime Puddle", "Kobold Thief", "Rusted Armor", "Goblin Grunt"];
    let tier_2 = ["Orc Berserker", "Shadow Stalker", "Dungeon Mimic", "Cave Troll", "Skeletal Mage"];
    let tier_3 = ["Stone Golem", "Vampire Lord", "Dark Paladin", "Beholder", "Void Elemental"];
    let bosses = [
        ("The Lich King", BossAbility::SilenceSuit(2)),
        ("Ancient Red Dragon", BossAbility::HandSizeMinusOne),
        ("King of Spades", BossAbility::DoubleTarget),
        ("The Dealer", BossAbility::PayToDiscard),
    ];

    let idx = unsafe { raylib::ffi::GetRandomValue(0, 4) } as usize;
    if ante < 3 { (tier_1[idx].to_string(), BossAbility::None) }
    else if ante < 6 { (tier_2[idx].to_string(), BossAbility::None) }
    else if ante < 8 { (tier_3[idx].to_string(), BossAbility::None) }
    else {
        let b_idx = unsafe { raylib::ffi::GetRandomValue(0, 3) } as usize;
        let (name, ability) = bosses[b_idx].clone();
        (name.to_string(), ability)
    }
}