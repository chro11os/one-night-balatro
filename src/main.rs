mod window_init;
mod draw_scene;
mod logic;
mod consts;
mod structures;

use raylib::prelude::*;
use structures::card::Card;
use structures::stats::BaseModifiers;
use consts::*;

fn main() {
    let (mut rl, thread) = window_init::initialize_window();
    let mut stats = BaseModifiers::default();

    // 1. Create the Full Deck
    let mut all_cards: Vec<Card> = Vec::with_capacity(52);
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

    // 2. Shuffle
    for i in 0..all_cards.len() {
        let swap_idx = unsafe { raylib::ffi::GetRandomValue(0, 51) } as usize;
        all_cards.swap(i, swap_idx);
    }

    // 3. Deal Hand
    let mut hand: Vec<Card> = Vec::with_capacity(8);
    // Initial deal uses logic's helper to ensure centering
    // But we can't call logic function easily here without dummying it out.
    // Let's just manually deal 8 cards using the logic we wrote in main previously, 
    // but now we rely on the logic loop to center them.
    for _ in 0..8 {
        if let Some(mut card) = all_cards.pop() {
            card.target_pos.y = HAND_Y_POS;
            hand.push(card);
        }
    }
    // Manually center initial hand
    let hand_size = hand.len();
    let gap = 10.0;
    let total_width = (hand_size as f32 * CARD_WIDTH) + ((hand_size - 1) as f32 * gap);
    let start_x = (SCREEN_WIDTH / 2.0) - (total_width / 2.0) + (CARD_WIDTH / 2.0);
    for (i, card) in hand.iter_mut().enumerate() {
        card.target_pos.x = start_x + (i as f32 * (CARD_WIDTH + gap));
    }

    stats.deck_count = all_cards.len() as i32;

    // --- MAIN LOOP ---
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // Pass 'hand' and 'all_cards' (the deck) as Mutable Vecs
        logic::update_game(&rl, &mut hand, &mut all_cards, &mut stats, dt);

        let mut d = rl.begin_drawing(&thread);
        draw_scene::draw_game(&mut d, &stats, &hand[..]);
    }
}