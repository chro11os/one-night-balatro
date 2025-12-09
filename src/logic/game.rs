use crate::poker;
use raylib::prelude::*;
use crate::structures::stats::{self, BaseModifiers, BossAbility, SortMode};
use crate::structures::card::Card;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;
use crate::score_manager;
use crate::structures::relic::{GameRelic, RelicEffect};
use crate::logic::metrics::GameMetrics;

pub fn update_game(rl: &RaylibHandle, hand: &mut Vec<Card>, deck: &mut Vec<Card>, stats: &mut BaseModifiers, dt: f32, state: &mut GameState, animation_state: &mut AnimationState) {

    if *state == GameState::Shop {
        if stats.shop_y_offset > 0.0 {
            stats.shop_y_offset -= 1500.0 * dt;
            if stats.shop_y_offset < 0.0 { stats.shop_y_offset = 0.0; }
        }
    }

    if stats.input_consumed {
        stats.input_consumed = false;
        return;
    }

    let mouse_pos = rl.get_mouse_position();
    let mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    let play_btn = Rectangle::new(PLAY_BTN_POS.x, PLAY_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);
    let discard_btn = Rectangle::new(DISC_BTN_POS.x, DISC_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);
    let sort_rank_btn = Rectangle::new(SORT_RANK_POS.x, SORT_RANK_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let sort_suit_btn = Rectangle::new(SORT_SUIT_POS.x, SORT_SUIT_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);

    // Update Tweens
    for card in hand.iter_mut() {
        if let Some(tween) = &mut card.tween {
            tween.update(dt);
            card.current_pos = tween.solve();
            if tween.is_finished() {
                card.current_pos = tween.end;
                card.tween = None;
            }
        }
    }

    match animation_state {
        AnimationState::Idle => {
            // Hand Helper
            // FIX: Clone cards to pass a slice of structs, not references to references
            let selected_cards: Vec<Card> = hand.iter().filter(|c| c.is_selected).cloned().collect();
            if !selected_cards.is_empty() {
                let rank = poker::get_hand_rank(&selected_cards, stats);
                stats.current_hand_text = format!("{:?}", rank);
            } else {
                stats.current_hand_text = String::new();
            }

            update_card_interaction(rl, hand, stats);

            if mouse_pressed {
                let selected_count = hand.iter().filter(|c| c.is_selected).count();

                if play_btn.check_collision_point_rec(mouse_pos) && stats.hands_remaining > 0 && selected_count > 0 && selected_count <= 5 {
                    for card in hand.iter_mut().filter(|c| c.is_selected) {
                        card.move_to(Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0), 0.4);
                        card.scale = Vector2::new(1.2, 1.2);
                    }
                    *animation_state = AnimationState::PlayingAnimation;
                    stats.score_timer = 0.6;
                    stats.input_consumed = true;
                }

                if discard_btn.check_collision_point_rec(mouse_pos) && stats.discards_remaining > 0 && selected_count > 0 && selected_count <= 5 {
                    stats.discards_remaining -= 1;
                    stats.game_metrics.log_discard(selected_count);
                    *animation_state = AnimationState::Discarding;
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
            update_hand_positions(hand);
        },

        AnimationState::PlayingAnimation => {
            stats.score_timer -= dt;

            if stats.score_timer <= 0.0 {
                let selected_cards: Vec<Card> = hand.iter().filter(|c| c.is_selected).cloned().collect();
                let rank = poker::get_hand_rank(&selected_cards, stats);
                let game_relics = stats.equipped_relics.clone();

                let (base_chips, base_mult) = poker::get_hand_base_score(rank);
                let result = score_manager::calculate_score(&selected_cards, &game_relics, base_chips, base_mult);

                stats.total_score += result.total;
                stats.round_score += result.total;
                stats.display_score += result.total as f32;
                stats.game_metrics.log_play(result.total);

                // FIX: Replace drain_filter (unstable) with stable retain logic
                // Move played cards to deck
                let mut i = 0;
                while i < hand.len() {
                    if hand[i].is_selected {
                        let mut c = hand.remove(i);
                        c.is_selected = false; // Reset state
                        deck.push(c);
                    } else {
                        i += 1;
                    }
                }

                stats.hands_remaining -= 1;

                if stats.round_score >= stats.target_score {
                    *state = GameState::BattleResult;
                } else if stats.hands_remaining == 0 {
                    *state = GameState::GameOver;
                } else {
                    refill_hand(hand, deck, stats);
                }

                *animation_state = AnimationState::Idle;
                stats.input_consumed = false;
            }
        },

        AnimationState::Discarding => {
            // FIX: Stable discard logic
            let mut i = 0;
            while i < hand.len() {
                if hand[i].is_selected {
                    let mut c = hand.remove(i);
                    c.is_selected = false;
                    deck.push(c);
                } else {
                    i += 1;
                }
            }
            refill_hand(hand, deck, stats);
            *animation_state = AnimationState::Idle;
        },

        _ => {}
    }
}

// Helpers
fn update_card_interaction(rl: &RaylibHandle, hand: &mut Vec<Card>, stats: &mut BaseModifiers) {
    let mouse_pos = rl.get_mouse_position();
    let mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let mut hovered_index = None;
    for (i, card) in hand.iter_mut().enumerate().rev() {
        let width = CARD_WIDTH * card.scale.x;
        let height = CARD_HEIGHT * card.scale.y;
        let rect = Rectangle::new(card.current_pos.x - width/2.0, card.current_pos.y - height/2.0, width, height);
        if rect.check_collision_point_rec(mouse_pos) {
            hovered_index = Some(i);
            break;
        }
    }
    for (i, card) in hand.iter_mut().enumerate() {
        card.is_hovered = Some(i) == hovered_index;
        if mouse_pressed && card.is_hovered {
            card.is_selected = !card.is_selected;
            stats.game_metrics.log_click(card.id as usize);
        }
    }
}

fn update_hand_positions(hand: &mut Vec<Card>) {
    let num_cards = hand.len();
    if num_cards == 0 { return; }
    let center_x = SCREEN_WIDTH / 2.0 + 50.0;
    let spacing = 90.0;
    let start_x = center_x - ((num_cards as f32 - 1.0) * spacing) / 2.0;
    let base_y = SCREEN_HEIGHT - 120.0;

    for (i, card) in hand.iter_mut().enumerate() {
        if card.tween.is_some() { continue; }
        let target_x = start_x + (i as f32 * spacing);
        let y_offset = if card.is_selected { 60.0 } else if card.is_hovered { 30.0 } else { 0.0 };
        let target_y = base_y - y_offset;
        if card.current_pos.distance_to(Vector2::new(target_x, target_y)) > 2.0 {
            card.move_to(Vector2::new(target_x, target_y), 0.15);
        }
    }
}

fn refill_hand(hand: &mut Vec<Card>, deck: &mut Vec<Card>, stats: &BaseModifiers) {
    while hand.len() < stats.hand_size as usize {
        if let Some(mut new_card) = deck.pop() {
            new_card.current_pos = Vector2::new(DECK_X, DECK_Y);
            hand.push(new_card);
        } else { break; }
    }
    sort_hand(hand, stats.current_sort);
}

fn sort_hand(hand: &mut Vec<Card>, mode: SortMode) {
    match mode {
        SortMode::Rank => hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit))),
        SortMode::Suit => hand.sort_by(|a, b| a.suit.cmp(&b.suit).then(b.value.cmp(&a.value))),
    }
}

// Add this if you haven't implemented start_next_round in game.rs yet
// (If you have, make sure it's consistent)
pub fn start_next_round(stats: &mut BaseModifiers, deck: &mut Vec<Card>) {
    use rand::{self, Rng, seq::SliceRandom};
    use crate::structures::enemy::{Enemy, EnemyAbility}; // Ensure correct path

    stats.round += 1;
    stats.current_hp = stats.max_hp;
    stats.hands_remaining = 4;
    stats.discards_remaining = 5;
    stats.round_score = 0;
    stats.display_score = 0.0;
    stats.shop_y_offset = SCREEN_HEIGHT;

    let mut rng = rand::thread_rng();
    if let Some(db) = &stats.enemy_database {
        let enemies: Vec<&Enemy> = db.values().collect();
        if let Some(chosen) = enemies.choose(&mut rng) {
            stats.current_enemy = Some((*chosen).clone());
            stats.enemy_name = chosen.name.clone();
            stats.target_score = chosen.hp;
            stats.active_ability = BossAbility::None; // Placeholder mapping
        }
    }

    deck.clear();
    let mut id_counter = 0;
    for suit in 0..4 {
        for value in 2..=14 {
            let mut card = Card::new(id_counter, DECK_X); // FIXED 2 args
            card.suit = suit;
            card.value = value;
            deck.push(card);
            id_counter += 1;
        }
    }
    deck.shuffle(&mut rng);
}