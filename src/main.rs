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

    // 1. Load Assets & All Game Data
    let mut assets = GameAssets::load(&mut rl, &thread);
    structures::data_loader::load_all_data(&mut assets);

    println!("> Loaded {} Relics", assets.relics_db.len());
    println!("> Loaded {} Runes", assets.runes_db.len());

    // 2. Transfer Data References to Stats
    stats.all_relics = assets.relics_db.clone();
    stats.all_consumables = assets.consumables_db.clone();
    stats.all_heirlooms = assets.heirlooms_db.clone();
    stats.enemy_database = Some(assets.enemies_db.clone());
    stats.available_runes = assets.runes_db.values().cloned().collect();

    let mut current_state = GameState::RuneSelect;
    let mut animation_state = AnimationState::Idle;
    let mut bench = bench::GameBench::new();

    // 3. Initialize Game State (Deck, Enemy, Hand)
    // --- FIX START ---
    let mut deck = Vec::new();
    let mut hand = Vec::new();

    // A. Generate Deck & Enemy via Logic (Uses the shuffle logic from game.rs)
    logic::game::start_next_round(&mut stats, &mut deck);
    stats.round = 1; // Reset round count to 1

    // B. Deal Initial Hand (So we don't start with 0 cards)
    while hand.len() < stats.hand_size as usize {
        if let Some(mut card) = deck.pop() {
            card.current_pos = Vector2::new(DECK_X, DECK_Y);
            hand.push(card);
        } else {
            break;
        }
    }

    // Sort for visual clarity
    stats.current_sort = structures::stats::SortMode::Rank;
    hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit)));
    // --- FIX END ---

    while !rl.window_should_close() {
        let frame_start = bench.start_frame();
        let dt = rl.get_frame_time();

        let update_start = Instant::now();
        match current_state {
            GameState::Menu => logic::update_menu(&rl, &mut current_state),
            GameState::RuneSelect => logic::update_rune_select(&rl, &mut current_state, &mut stats),
            GameState::Playing => logic::update_game(&rl, &mut hand, &mut deck, &mut stats, dt, &mut current_state, &mut animation_state),
            GameState::BattleResult => logic::update_battle_result(&mut rl, &mut current_state, &mut stats),
            GameState::StatsMenu => logic::update_stats_menu(&rl, &mut current_state, &mut stats),
            GameState::Shop => logic::update_shop(&mut rl, &mut current_state, &mut stats, &mut deck),
            GameState::GameOver => {
                if rl.is_key_pressed(KeyboardKey::KEY_R) {
                    // Reset Logic
                    let saved_db = stats.enemy_database.clone();
                    let saved_relics = stats.all_relics.clone();
                    let saved_runes = stats.available_runes.clone();

                    stats = BaseModifiers::default();
                    stats.enemy_database = saved_db;
                    stats.all_relics = saved_relics;
                    stats.available_runes = saved_runes;

                    // Re-init deck & hand using the same logic
                    deck.clear();
                    hand.clear();
                    logic::game::start_next_round(&mut stats, &mut deck);
                    stats.round = 1;

                    while hand.len() < stats.hand_size as usize {
                        if let Some(mut card) = deck.pop() {
                            card.current_pos = Vector2::new(DECK_X, DECK_Y);
                            hand.push(card);
                        } else { break; }
                    }
                    hand.sort_by(|a, b| b.value.cmp(&a.value).then(a.suit.cmp(&b.suit)));

                    stats.deck_count = deck.len() as i32;
                    current_state = GameState::RuneSelect;
                }
            }
            _ => {}
        }
        bench.record_update(update_start.elapsed());

        stats.update_screen_shake(dt);
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