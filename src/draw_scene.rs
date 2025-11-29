use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::card::Card;
use crate::structures::assets::GameAssets;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;

// ... [draw_scene and draw_sidebar remain unchanged] ...
pub fn draw_scene(d: &mut RaylibDrawHandle, stats: &BaseModifiers, hand: &[Card], state: &GameState, assets: &GameAssets, anim: &AnimationState) {
    match state {
        GameState::Playing => {
            d.clear_background(Color::new(34, 139, 34, 255));
            for y in (0..SCREEN_HEIGHT as i32).step_by(4) {
                d.draw_line(0, y, SCREEN_WIDTH as i32, y, Color::BLACK.alpha(0.1));
            }
            draw_sidebar(d, stats, anim);
            draw_game_area(d, hand, assets, stats);
        },
        _ => d.clear_background(NEU_BG),
    }
    d.draw_fps(10, 10);
}

fn draw_sidebar(d: &mut RaylibDrawHandle, stats: &BaseModifiers, anim: &AnimationState) {
    let sb_w = SIDEBAR_WIDTH;
    let h = SCREEN_HEIGHT;
    d.draw_rectangle(0, 0, sb_w as i32, h as i32, NEU_BLACK);
    d.draw_line_ex(Vector2::new(sb_w, 0.0), Vector2::new(sb_w, h), 4.0, Color::BLACK);
    let pad = 10.0;

    // Blind
    let blind_rect = Rectangle::new(pad, pad, sb_w - pad*2.0, 120.0);
    d.draw_rectangle_rounded(blind_rect, 0.1, 4, NEU_BLUE);
    d.draw_rectangle_rounded_lines_ex(blind_rect, 0.1, 4, 3.0, Color::BLACK);
    d.draw_text("Small Blind", (pad + 15.0) as i32, (pad + 10.0) as i32, 24, Color::WHITE);
    d.draw_text("Score at least", (pad + 60.0) as i32, (pad + 50.0) as i32, 20, Color::WHITE);
    d.draw_text(&format!("{}", stats.target_score), (pad + 60.0) as i32, (pad + 75.0) as i32, 40, NEU_RED);

    // Round Score
    let round_y = 140.0;
    let round_rect = Rectangle::new(pad, round_y, sb_w - pad*2.0, 70.0);
    d.draw_rectangle_rounded(round_rect, 0.1, 4, NEU_BLACK);
    d.draw_rectangle_rounded_lines_ex(round_rect, 0.1, 4, 2.0, Color::GRAY);
    d.draw_text("Round Score", (pad + 20.0) as i32, (round_y + 10.0) as i32, 16, Color::GRAY);

    let mut score_color = Color::WHITE;
    let mut score_size = 40;
    let mut score_pos_x = (pad + 20.0) as i32;
    let mut score_pos_y = (round_y + 25.0) as i32;
    if *anim == AnimationState::Scoring {
        score_color = NEU_ORANGE;
        let time = d.get_time() as f32;
        score_size = 45 + (time * 20.0).sin() as i32 * 2;
        score_pos_x += (time * 50.0).sin() as i32 * 2;
        score_pos_y += (time * 30.0).cos() as i32 * 2;
    }
    d.draw_text(&format!("{}", stats.display_score as i32), score_pos_x, score_pos_y, score_size, score_color);

    // Hand Calc
    let score_y = 230.0;
    let score_rect = Rectangle::new(pad, score_y, sb_w - pad*2.0, 150.0);
    d.draw_rectangle_rounded(score_rect, 0.1, 4, Color::new(20, 20, 20, 255));
    d.draw_rectangle_rounded_lines_ex(score_rect, 0.1, 4, 2.0, Color::GRAY);
    let hand_name = match stats.hand_rank {
        Some(r) => format!("{:?}", r),
        None => "Choose Cards".to_string()
    };
    d.draw_text(&hand_name, (pad + 20.0) as i32, (score_y + 15.0) as i32, 24, Color::WHITE);
    let calc_box_y = score_y + 50.0;
    d.draw_rectangle_rounded(Rectangle::new(pad + 10.0, calc_box_y, 80.0, 60.0), 0.2, 4, NEU_BLUE);
    d.draw_text(&format!("{}", stats.chips), (pad + 20.0) as i32, (calc_box_y + 15.0) as i32, 30, Color::WHITE);
    d.draw_text("X", (pad + 100.0) as i32, (calc_box_y + 15.0) as i32, 30, Color::RED);
    d.draw_rectangle_rounded(Rectangle::new(pad + 130.0, calc_box_y, 80.0, 60.0), 0.2, 4, NEU_RED);
    d.draw_text(&format!("{}", stats.mult), (pad + 145.0) as i32, (calc_box_y + 15.0) as i32, 30, Color::WHITE);

    // Stats
    let stats_y = 500.0;
    draw_stat_box(d, "Hands", stats.hands_remaining, NEU_BLUE, pad, stats_y);
    draw_stat_box(d, "Discards", stats.discards_remaining, NEU_RED, pad + 110.0, stats_y);

    let money_y = stats_y + 80.0;
    let money_rect = Rectangle::new(pad + 50.0, money_y, 120.0, 60.0);
    d.draw_rectangle_rounded(money_rect, 0.2, 4, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(money_rect, 0.2, 4, 3.0, Color::BLACK);
    d.draw_text(&format!("$ {}", stats.money), (pad + 75.0) as i32, (money_y + 15.0) as i32, 30, Color::WHITE);

    let ar_y = money_y + 80.0;
    d.draw_rectangle_rounded(Rectangle::new(pad, ar_y, 100.0, 50.0), 0.2, 4, NEU_BLACK);
    d.draw_text("Ante", (pad + 25.0) as i32, (ar_y + 5.0) as i32, 20, Color::GRAY);
    d.draw_text(&format!("{}/8", stats.ante), (pad + 30.0) as i32, (ar_y + 25.0) as i32, 20, Color::WHITE);
    d.draw_rectangle_rounded(Rectangle::new(pad + 110.0, ar_y, 100.0, 50.0), 0.2, 4, NEU_BLACK);
    d.draw_text("Round", (pad + 130.0) as i32, (ar_y + 5.0) as i32, 20, Color::GRAY);
    d.draw_text(&format!("{}", stats.round), (pad + 150.0) as i32, (ar_y + 25.0) as i32, 20, Color::WHITE);
}

fn draw_stat_box(d: &mut RaylibDrawHandle, label: &str, val: i32, color: Color, x: f32, y: f32) {
    let rect = Rectangle::new(x, y, 100.0, 70.0);
    d.draw_rectangle_rounded(rect, 0.1, 4, NEU_BLACK);
    d.draw_rectangle_rounded_lines_ex(rect, 0.1, 4, 2.0, color);
    d.draw_text(label, (x + 15.0) as i32, (y + 10.0) as i32, 16, color);
    d.draw_text(&format!("{}", val), (x + 40.0) as i32, (y + 35.0) as i32, 30, Color::WHITE);
}

fn draw_game_area(d: &mut RaylibDrawHandle, hand: &[Card], assets: &GameAssets, _stats: &BaseModifiers) {
    let center_x = SIDEBAR_WIDTH + (SCREEN_WIDTH - SIDEBAR_WIDTH) / 2.0;

    // Buttons Y Position
    let btn_y = 660.0;
    let sort_y = 620.0; // Above play buttons

    // --- SORT BUTTONS ---
    // Rank (Orange)
    let sort_rank_rect = Rectangle::new(center_x - SORT_BTN_WIDTH - 5.0, sort_y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    d.draw_rectangle_rounded(sort_rank_rect, 0.2, 4, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(sort_rank_rect, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text("Rank", (sort_rank_rect.x + 20.0) as i32, (sort_rank_rect.y + 8.0) as i32, 16, Color::WHITE);

    // Suit (Blue)
    let sort_suit_rect = Rectangle::new(center_x + 5.0, sort_y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    d.draw_rectangle_rounded(sort_suit_rect, 0.2, 4, NEU_BLUE);
    d.draw_rectangle_rounded_lines_ex(sort_suit_rect, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text("Suit", (sort_suit_rect.x + 25.0) as i32, (sort_suit_rect.y + 8.0) as i32, 16, Color::WHITE);

    // --- ACTION BUTTONS ---
    // Play Button
    let play_rect = Rectangle::new(center_x - BTN_WIDTH - 10.0, btn_y, BTN_WIDTH, BTN_HEIGHT);
    d.draw_rectangle_rounded(play_rect, 0.2, 4, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(play_rect, 0.2, 4, 3.0, Color::BLACK);
    d.draw_text("PLAY", (play_rect.x + 35.0) as i32, (play_rect.y + 12.0) as i32, 20, Color::WHITE);

    // Discard Button
    let disc_rect = Rectangle::new(center_x + 10.0, btn_y, BTN_WIDTH, BTN_HEIGHT);
    d.draw_rectangle_rounded(disc_rect, 0.2, 4, NEU_RED);
    d.draw_rectangle_rounded_lines_ex(disc_rect, 0.2, 4, 3.0, Color::BLACK);
    d.draw_text("DISCARD", (disc_rect.x + 15.0) as i32, (disc_rect.y + 12.0) as i32, 20, Color::WHITE);

    // Deck
    d.draw_texture_pro(
        assets.tex_cards.get("card_back").unwrap_or(assets.tex_cards.values().next().unwrap()),
        Rectangle::new(0.0, 0.0, CARD_WIDTH, CARD_HEIGHT),
        Rectangle::new(DECK_X, DECK_Y, CARD_WIDTH, CARD_HEIGHT),
        Vector2::zero(), 0.0, Color::WHITE
    );
    d.draw_text("Deck", (DECK_X) as i32, (DECK_Y + 200.0) as i32, 20, Color::WHITE);

    for card in hand {
        draw_single_card(d, card, assets);
    }
}

// ... [draw_single_card remains unchanged] ...
fn draw_single_card(d: &mut RaylibDrawHandle, card: &Card, assets: &GameAssets) {
    let width = CARD_WIDTH * card.scale;
    let height = CARD_HEIGHT * card.scale;
    let dest_rect = Rectangle::new(card.current_pos.x, card.current_pos.y, width, height);

    let suit_str = match card.suit { 0 => "clubs", 1 => "diamonds", 2 => "hearts", 3 => "spades", _ => "" };
    let rank_str = match card.value { 11 => "J".to_string(), 12 => "Q".to_string(), 13 => "K".to_string(), 14 => "A".to_string(), v => format!("{:02}", v) };
    let key = format!("card_{}_{}", suit_str, rank_str);

    if let Some(texture) = assets.tex_cards.get(&key) {
        let origin = Vector2::new(width / 2.0, height / 2.0);
        d.draw_texture_pro(
            texture,
            Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
            Rectangle::new(dest_rect.x + 6.0, dest_rect.y + 6.0, width, height),
            origin,
            card.rotation * 57.29,
            Color::BLACK.alpha(0.4)
        );
        let tint = if card.is_hovered { Color::WHITE } else { Color::WHITE };
        d.draw_texture_pro(
            texture,
            Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
            Rectangle::new(dest_rect.x, dest_rect.y, width, height),
            origin,
            card.rotation * 57.29,
            tint
        );
    }
}