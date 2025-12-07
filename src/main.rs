mod window_init;
mod drawing;
mod logic;
mod consts;
mod structures;
mod poker;
mod bench;
mod utils;
mod score_manager;

use raylib::prelude::*;
use structures::card::Card;
use structures::stats::BaseModifiers;
use structures::assets::GameAssets;
use structures::state::{GameState, AnimationState};
use consts::*;
use std::time::Instant;

fn main() {
    let (mut rl, thread) = window_init::initialize_window();
    let mut stats = BaseModifiers::default();
    let assets = GameAssets::load(&mut rl, &thread);

    println!("Loading Game Data...");
    stats.enemy_database = Some(structures::data_loader::load_enemies());
    stats.all_relics = structures::data_loader::load_relics();
    stats.available_runes = structures::data_loader::load_runes();

    println!("> Loaded {} Relics", stats.all_relics.len());
    println!("> Loaded {} Runes", stats.available_runes.len());

    // FIX: Start in RuneSelect so we see the new screen immediately!
    let mut current_state = GameState::RuneSelect;
    let mut animation_state = AnimationState::Idle;
    let mut bench = bench::GameBench::new();

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

    let (mut hand, mut deck) = setup_game(stats.hand_size);
    stats.deck_count = deck.len() as i32;

    while !rl.window_should_close() && current_state != GameState::Exit {
        let frame_start = bench.start_frame();
        let dt = rl.get_frame_time();
        let total_time = rl.get_time() as f32;
        let update_start = Instant::now();

        match current_state {
            GameState::Menu => logic::update_menu(&rl, &mut current_state),
            GameState::Playing => logic::update_game(&rl, &mut hand, &mut deck, &mut stats, dt, &mut current_state, &mut animation_state, total_time),
            GameState::RuneSelect => logic::update_rune_select(&rl, &mut current_state, &mut stats),
            GameState::Shop => logic::update_shop(&mut rl, &mut current_state, &mut stats, &mut hand, &mut deck),
            GameState::StatsMenu => logic::update_stats_menu(&rl, &mut current_state, &mut stats),
            GameState::BattleResult => logic::update_battle_result(&mut rl, &mut current_state, &mut stats),
            GameState::Settings => if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) { current_state = GameState::Menu; },
            GameState::GameOver => {
                if rl.is_key_pressed(KeyboardKey::KEY_R) {
                    let saved_enemies = stats.enemy_database.clone();
                    let saved_relics = stats.all_relics.clone();
                    stats = BaseModifiers::default();
                    stats.enemy_database = saved_enemies;
                    stats.all_relics = saved_relics;
                    stats.available_runes = structures::data_loader::load_runes(); // Reload runes too

                    let (new_hand, new_deck) = setup_game(stats.hand_size);
                    hand = new_hand;
                    deck = new_deck;
                    stats.deck_count = deck.len() as i32;
                    current_state = GameState::RuneSelect; // Restart to RuneSelect
                }
            }
            _ => {}
        }
        bench.record_update(update_start.elapsed());

        stats.update_screen_shake(dt); // Update screen shake trauma and offset

        // Update cached strings after all logic for the frame has run
        stats.update_cached_strings();

        let draw_start = Instant::now();
        let mut d = rl.begin_drawing(&thread);
        drawing::draw_scene(&mut d, &stats, &hand, &current_state, &assets, &animation_state);
        bench.record_draw(draw_start.elapsed());
        drop(d);
        bench.end_frame(frame_start);

        if let Some(report) = bench.report() {
            println!("{}", report);
        }
    }
}