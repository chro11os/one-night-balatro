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

    let setup_game = || -> (Vec<Card>, Vec<Card>) {
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
        for _ in 0..6 {
            if let Some(mut card) = all_cards.pop() {
                card.target_pos.y = HAND_Y_POS;
                hand.push(card);
            }
        }
        let hand_size = hand.len();
        let gap = 5.0;
        let total_width = (hand_size as f32 * CARD_WIDTH) + ((hand_size - 1) as f32 * gap);
        let start_x = (SCREEN_WIDTH / 2.0) - (total_width / 2.0) + (CARD_WIDTH / 2.0);
        for (i, card) in hand.iter_mut().enumerate() {
            card.target_pos.x = start_x + (i as f32 * (CARD_WIDTH + gap));
        }
        (hand, all_cards)
    };

    let (mut hand, mut deck) = setup_game();
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
            GameState::Settings => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    current_state = GameState::Menu;
                }
            }
            GameState::GameOver => {
                if rl.is_key_pressed(KeyboardKey::KEY_R) {
                    let (new_hand, new_deck) = setup_game();
                    hand = new_hand;
                    deck = new_deck;
                    stats = BaseModifiers::default();
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