mod window_init;
mod draw_scene;
mod logic;
mod consts;
mod structures;
mod poker;

use raylib::prelude::*;
use structures::card::Card;
use structures::stats::BaseModifiers;
use structures::assets::GameAssets;
use structures::state::{GameState, AnimationState};
use consts::*;

fn main() {
    let (mut rl, thread) = window_init::initialize_window();
    let mut stats = BaseModifiers::default();
    let assets = GameAssets::load(&mut rl, &thread);

    let mut current_state = GameState::Menu;
    let mut animation_state = AnimationState::Idle;

    // FIX: Pass 'hand_size' as an argument so we don't capture 'stats'
    let setup_game = |hand_size: i32| -> (Vec<Card>, Vec<Card>) {
        let mut all_cards = Vec::with_capacity(52);
        let mut id_counter = 0;
        for suit in 0..4 {
            for val in 2..15 {
                let mut card = Card::new(id_counter, DECK_X, DECK_Y);
                card.suit = suit;
                card.value = val;
                all_cards.push(card);
                id_counter += 1;
            }
        }
        for i in 0..all_cards.len() {
            let swap_idx = unsafe { raylib::ffi::GetRandomValue(0, 51) } as usize;
            all_cards.swap(i, swap_idx);
        }
        let mut hand = Vec::with_capacity(6);
        for _ in 0..hand_size {
            if let Some(mut card) = all_cards.pop() {
                card.target_pos.y = HAND_Y_POS;
                hand.push(card);
            }
        }
        (hand, all_cards)
    };

    // Initial setup
    let (mut hand, mut deck) = setup_game(stats.hand_size);
    stats.deck_count = deck.len() as i32;

    while !rl.window_should_close() && current_state != GameState::Exit {
        let dt = rl.get_frame_time();
        let total_time = rl.get_time() as f32;

        match current_state {
            GameState::Menu => {
                logic::update_menu(&rl, &mut current_state);
            }
            GameState::Playing => {
                logic::update_game(&rl, &mut hand, &mut deck, &mut stats, dt, &mut current_state, &mut animation_state, total_time);
            }
            GameState::RuneSelect => {
                logic::update_rune_select(&rl, &mut current_state, &mut stats);
            }
            GameState::Shop => {
                logic::update_shop(&rl, &mut current_state, &mut stats, &mut hand, &mut deck);
            }
            GameState::StatsMenu => {
                logic::update_stats_menu(&rl, &mut current_state, &mut stats);
            }
            GameState::BattleResult => {
                logic::update_battle_result(&rl, &mut current_state, &mut stats);
            }
            GameState::Settings => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    current_state = GameState::Menu;
                }
            }
            GameState::GameOver => {
                if rl.is_key_pressed(KeyboardKey::KEY_R) {
                    // Reset stats
                    stats = BaseModifiers::default();
                    // Re-deal using default hand size
                    let (new_hand, new_deck) = setup_game(stats.hand_size);
                    hand = new_hand;
                    deck = new_deck;
                    stats.deck_count = deck.len() as i32;
                    current_state = GameState::Playing;
                }
            }
            _ => {}
        }

        let mut d = rl.begin_drawing(&thread);
        draw_scene::draw_scene(&mut d, &stats, &hand[..], &current_state, &assets, &animation_state);
    }
}