use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::card::Card;
use crate::consts::*;

pub fn draw_game(d: &mut RaylibDrawHandle, stats: &BaseModifiers, hand: &[Card]) {
    d.clear_background(NEU_BG);

    d.draw_fps(10, 10);

    draw_ui(d, stats);
    draw_buttons(d, stats); // New
    draw_deck(d, stats);

    for card in hand {
        draw_single_card(d, card);
    }
}

fn draw_buttons(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    // --- PLAY HAND BUTTON ---
    let play_rect = Rectangle::new(PLAY_BTN_POS.x, PLAY_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);
    // Shadow
    d.draw_rectangle_rounded(
        Rectangle::new(play_rect.x + 4.0, play_rect.y + 4.0, play_rect.width, play_rect.height),
        0.2, 10, NEU_BLACK.alpha(0.5)
    );
    // Body
    d.draw_rectangle_rounded(play_rect, 0.2, 10, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(play_rect, 0.2, 10, 3.0, NEU_BLACK);
    d.draw_text("PLAY HAND", (play_rect.x + 25.0) as i32, (play_rect.y + 20.0) as i32, 20, Color::WHITE);

    // --- DISCARD BUTTON ---
    let disc_rect = Rectangle::new(DISC_BTN_POS.x, DISC_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);
    // Shadow
    d.draw_rectangle_rounded(
        Rectangle::new(disc_rect.x + 4.0, disc_rect.y + 4.0, disc_rect.width, disc_rect.height),
        0.2, 10, NEU_BLACK.alpha(0.5)
    );
    // Body
    d.draw_rectangle_rounded(disc_rect, 0.2, 10, NEU_RED);
    d.draw_rectangle_rounded_lines_ex(disc_rect, 0.2, 10, 3.0, NEU_BLACK);
    d.draw_text("DISCARD", (disc_rect.x + 35.0) as i32, (disc_rect.y + 20.0) as i32, 20, Color::WHITE);
}

fn draw_deck(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    let visual_layers = (stats.deck_count as f32 / 5.0).ceil() as i32;
    let max_layers = 6;
    let layers = visual_layers.min(max_layers);

    for i in 0..layers {
        let off = (layers - i) as f32 * 2.0;
        let rect = Rectangle::new(DECK_X - off, DECK_Y - off, CARD_WIDTH, CARD_HEIGHT);

        d.draw_rectangle_rec(rect, Color::WHITE);
        d.draw_rectangle_lines_ex(rect, 1.0, NEU_BLACK);

        let inner_rect = Rectangle::new(rect.x + 4.0, rect.y + 4.0, rect.width - 8.0, rect.height - 8.0);
        d.draw_rectangle_rec(inner_rect, NEU_RED);
    }

    // Count
    let text = format!("{}", stats.deck_count);
    d.draw_text(&text, (DECK_X + 30.0) as i32, (DECK_Y + 40.0) as i32, 30, Color::WHITE);
    d.draw_text("DECK", (DECK_X + 15.0) as i32, (DECK_Y + CARD_HEIGHT + 10.0) as i32, 20, Color::WHITE);
}

fn draw_ui(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    let panel_x = 20.0;
    let panel_y = 100.0;
    let panel_w = 220.0;
    let panel_h = 400.0;

    // Sidebar Background
    d.draw_rectangle_rounded(Rectangle::new(panel_x, panel_y, panel_w, panel_h), 0.1, 10, PANEL_BG);
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(panel_x, panel_y, panel_w, panel_h), 0.1, 10, 3.0, NEU_BLACK);

    // Score
    let score_box_y = panel_y + 20.0;
    d.draw_rectangle_rounded(Rectangle::new(panel_x + 10.0, score_box_y, 200.0, 60.0), 0.2, 10, NEU_RED);
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(panel_x + 10.0, score_box_y, 200.0, 60.0), 0.2, 10, 2.0, NEU_BLACK);
    d.draw_text("Round Score", (panel_x + 20.0) as i32, (score_box_y + 10.0) as i32, 20, Color::WHITE);
    d.draw_text(&format!("{}", stats.total_score), (panel_x + 20.0) as i32, (score_box_y + 35.0) as i32, 30, Color::WHITE);

    // Run Info (Hands/Discards)
    let info_y = panel_y + 250.0;

    // Hands
    d.draw_rectangle_rounded(Rectangle::new(panel_x + 10.0, info_y, 90.0, 60.0), 0.2, 10, NEU_BLUE);
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(panel_x + 10.0, info_y, 90.0, 60.0), 0.2, 10, 2.0, NEU_BLACK);
    d.draw_text("Hands", (panel_x + 20.0) as i32, (info_y + 5.0) as i32, 10, Color::WHITE);
    d.draw_text(&format!("{}", stats.hands_remaining), (panel_x + 40.0) as i32, (info_y + 20.0) as i32, 30, Color::WHITE);

    // Discards
    d.draw_rectangle_rounded(Rectangle::new(panel_x + 110.0, info_y, 90.0, 60.0), 0.2, 10, NEU_RED);
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(panel_x + 110.0, info_y, 90.0, 60.0), 0.2, 10, 2.0, NEU_BLACK);
    d.draw_text("Discards", (panel_x + 120.0) as i32, (info_y + 5.0) as i32, 10, Color::WHITE);
    d.draw_text(&format!("{}", stats.discards_remaining), (panel_x + 140.0) as i32, (info_y + 20.0) as i32, 30, Color::WHITE);
}

fn draw_single_card(d: &mut RaylibDrawHandle, card: &Card) {
    let width = CARD_WIDTH * card.scale;
    let height = CARD_HEIGHT * card.scale;
    let x = card.current_pos.x - width / 2.0;
    let y = card.current_pos.y - height / 2.0;
    let rect = Rectangle::new(x, y, width, height);

    // Shadow
    let shadow_dist = if card.is_hovered { 8.0 } else { 4.0 };
    d.draw_rectangle((x + shadow_dist) as i32, (y + shadow_dist) as i32, width as i32, height as i32, NEU_BLACK.alpha(0.5));

    // Body
    d.draw_rectangle_rec(rect, CARD_BG);
    let border_color = if card.is_hovered { NEU_BLUE } else { NEU_BLACK };
    d.draw_rectangle_lines_ex(rect, if card.is_hovered { 3.0 } else { 2.0 }, border_color);

    // Content
    let rank_str = match card.value {
        11 => "J", 12 => "Q", 13 => "K", 14 => "A", _ => "",
    };
    let display_text = if rank_str == "" { format!("{}", card.value) } else { rank_str.to_string() };
    let (suit_color, suit_char) = match card.suit {
        0 => (SUIT_RED, "♥"), 1 => (SUIT_RED, "♦"),
        2 => (SUIT_BLACK, "♠"), _ => (SUIT_BLACK, "♣"),
    };

    d.draw_text(display_text.as_str(), (x + 8.0) as i32, (y + 8.0) as i32, 20, suit_color);
    let center_size = 40;
    let text_w = d.measure_text(suit_char, center_size);
    d.draw_text(suit_char, (x + width/2.0) as i32 - (text_w / 2), (y + height/2.0) as i32 - (center_size / 2), center_size, suit_color);
}