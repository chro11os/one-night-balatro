use raylib::prelude::*;
use crate::structures::stats::{BaseModifiers, BossAbility};
use crate::structures::card::Card;
use crate::structures::assets::GameAssets;
use crate::structures::state::AnimationState;
use crate::consts::*;
use crate::drawing::ui_elements::get_button_offset;


pub fn draw_game_area(d: &mut RaylibDrawHandle, hand: &[Card], assets: &GameAssets, _stats: &BaseModifiers) {
    // 1. Create a list of references
    let mut draw_order: Vec<&Card> = hand.iter().collect();

    // 2. Sort: Idle cards (0) first, Moving/Hovered/Selected cards (1) last
    draw_order.sort_by_key(|c| {
        if c.tween.is_some() || c.is_selected || c.is_hovered {
            1
        } else {
            0
        }
    });

    println!("Drawing {} cards...", draw_order.len()); // DEBUG CHECK 1

    for (i, card) in draw_order.iter().enumerate() {
        // DEBUG CHECK 2: Print first card details
        if i == 0 {
            println!("Card[0]: Pos: {:?}, Scale: {:?}, Tween Active: {}", 
                card.current_pos, card.scale, card.tween.is_some());
        }
        draw_single_card(d, card, assets);
    }
}

pub fn draw_action_panel(d: &mut RaylibDrawHandle, stats: &BaseModifiers, _anim: &AnimationState, assets: &GameAssets, hand: &[Card]) {
    let x = A_PANEL_X;
    let y = A_PANEL_Y;
    let w = A_PANEL_W;
    let h = A_PANEL_H;
    d.draw_rectangle_rounded(Rectangle::new(x, y, w, h), 0.1, 4, NEU_BLACK.alpha(0.9));
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(x, y, w, h), 0.1, 4, 3.0, NEU_RED);

    let play_rect = Rectangle::new(PLAY_BTN_POS.x, PLAY_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);
    let disc_rect = Rectangle::new(DISC_BTN_POS.x, DISC_BTN_POS.y, BTN_WIDTH, BTN_HEIGHT);

    // Visual disable logic: > 0 AND <= 5
    let selected_count = hand.iter().filter(|c| c.is_selected).count();
    let can_play = selected_count > 0 && selected_count <= 5 && stats.hands_remaining > 0;
    let can_disc = selected_count > 0 && selected_count <= 5 && stats.discards_remaining > 0;

    let (off_p, shad_p) = get_button_offset(d, play_rect);
    let play_tint = if can_play { Color::WHITE } else { Color::GRAY };

    d.draw_texture_pro(&assets.tex_btn_play, Rectangle::new(0.0,0.0, assets.tex_btn_play.width as f32, assets.tex_btn_play.height as f32), Rectangle::new(play_rect.x, play_rect.y+shad_p, play_rect.width, play_rect.height), Vector2::zero(), 0.0, Color::BLACK.alpha(0.5));
    d.draw_texture_pro(&assets.tex_btn_play, Rectangle::new(0.0,0.0, assets.tex_btn_play.width as f32, assets.tex_btn_play.height as f32), Rectangle::new(play_rect.x, play_rect.y+off_p, play_rect.width, play_rect.height), Vector2::zero(), 0.0, play_tint);

    // FONT UPDATE
    d.draw_text_ex(&assets.font_main, "PLAY", Vector2::new(play_rect.x + 40.0, play_rect.y + 15.0 + off_p), 24.0, 1.0, play_tint);

    let (off_d, shad_d) = get_button_offset(d, disc_rect);
    let disc_tint = if can_disc { Color::WHITE } else { Color::GRAY };

    d.draw_texture_pro(&assets.tex_btn_discard, Rectangle::new(0.0,0.0, assets.tex_btn_discard.width as f32, assets.tex_btn_discard.height as f32), Rectangle::new(disc_rect.x, disc_rect.y+shad_d, disc_rect.width, disc_rect.height), Vector2::zero(), 0.0, Color::BLACK.alpha(0.5));
    d.draw_texture_pro(&assets.tex_btn_discard, Rectangle::new(0.0,0.0, assets.tex_btn_discard.width as f32, assets.tex_btn_discard.height as f32), Rectangle::new(disc_rect.x, disc_rect.y+off_d, disc_rect.width, disc_rect.height), Vector2::zero(), 0.0, disc_tint);

    // FONT UPDATE
    d.draw_text_ex(&assets.font_main, "DISC", Vector2::new(disc_rect.x + 40.0, disc_rect.y + 15.0 + off_d), 24.0, 1.0, disc_tint);

    d.draw_text_ex(&assets.font_main, &stats.hands_remaining_text, Vector2::new(play_rect.x + 10.0, play_rect.y + 65.0), 18.0, 1.0, NEU_BLUE);
    d.draw_text_ex(&assets.font_main, &stats.discards_remaining_text, Vector2::new(disc_rect.x + 10.0, disc_rect.y + 65.0), 18.0, 1.0, NEU_RED);

    let score_y = SCORE_BOX_Y;
    let score_h = h - (score_y - y) - 15.0;
    let score_rect = Rectangle::new(x + 15.0, score_y, w - 30.0, score_h);
    d.draw_rectangle_rounded(score_rect, 0.1, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded_lines_ex(score_rect, 0.1, 4, 2.0, Color::GRAY);

    let box_w = 110.0;
    let box_h = 70.0;
    let chips_x = x + 25.0;
    let chips_y = score_y + 20.0;
    d.draw_texture_pro(&assets.tex_banner, Rectangle::new(0.0,0.0, assets.tex_banner.width as f32, assets.tex_banner.height as f32), Rectangle::new(chips_x, chips_y, box_w, box_h), Vector2::zero(), 0.0, NEU_BLUE);

    // FONT UPDATE
    d.draw_text_ex(&assets.font_main, &stats.chips_text, Vector2::new(chips_x + 15.0, chips_y + 20.0), 34.0, 1.0, PARCHMENT);
    d.draw_text_ex(&assets.font_main, "X", Vector2::new(chips_x + box_w + 10.0, chips_y + 20.0), 34.0, 1.0, Color::WHITE);

    let mult_x = chips_x + box_w + 40.0;
    d.draw_texture_pro(&assets.tex_banner, Rectangle::new(0.0,0.0, assets.tex_banner.width as f32, assets.tex_banner.height as f32), Rectangle::new(mult_x, chips_y, box_w, box_h), Vector2::zero(), 0.0, NEU_RED);

    // FONT UPDATE
    d.draw_text_ex(&assets.font_main, &stats.mult_text, Vector2::new(mult_x + 15.0, chips_y + 20.0), 34.0, 1.0, PARCHMENT);
}

pub fn draw_player_panel(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    let x = P_PANEL_X;
    let y = P_PANEL_Y;
    let w = P_PANEL_W;
    let h = P_PANEL_H;
    d.draw_rectangle_rounded(Rectangle::new(x, y, w, h), 0.1, 4, NEU_BLACK.alpha(0.9));
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(x, y, w, h), 0.1, 4, 3.0, NEU_BLUE);

    let hp_bar_h = 30.0;
    let hp_margin = 20.0;
    let hp_y = y + 40.0;
    let hp_w = w - hp_margin * 2.0;

    // FONT UPDATE
    d.draw_text_ex(&assets.font_main, "HP", Vector2::new(x + hp_margin, y + 15.0), 20.0, 1.0, PARCHMENT);
    d.draw_rectangle((x + hp_margin) as i32, hp_y as i32, hp_w as i32, hp_bar_h as i32, Color::BLACK);
    let hp_pct = (stats.current_hp as f32 / stats.max_hp as f32).clamp(0.0, 1.0);
    d.draw_rectangle((x + hp_margin) as i32, hp_y as i32, (hp_w * hp_pct) as i32, hp_bar_h as i32, NEU_RED);
    d.draw_text_ex(&assets.font_main, &stats.hp_text, Vector2::new(x + hp_margin + 10.0, hp_y + 5.0), 20.0, 1.0, Color::WHITE);

    let row2_y = hp_y + hp_bar_h + 20.0;
    let box_w = (hp_w - 10.0) / 2.0;
    let box_h = 50.0;
    d.draw_rectangle_lines((x + hp_margin) as i32, row2_y as i32, box_w as i32, box_h as i32, NEU_ORANGE);
    d.draw_text_ex(&assets.font_main, "MONEY", Vector2::new(x + hp_margin + 5.0, row2_y + 5.0), 16.0, 1.0, NEU_ORANGE);
    d.draw_text_ex(&assets.font_main, &stats.money_text, Vector2::new(x + hp_margin + 20.0, row2_y + 20.0), 24.0, 1.0, PARCHMENT);

    let lvl_x = x + hp_margin + box_w + 10.0;
    d.draw_rectangle_lines(lvl_x as i32, row2_y as i32, box_w as i32, box_h as i32, NEU_BLUE);
    d.draw_text_ex(&assets.font_main, "LEVEL", Vector2::new(lvl_x + 5.0, row2_y + 5.0), 16.0, 1.0, NEU_BLUE);
    d.draw_text_ex(&assets.font_main, &stats.level_text, Vector2::new(lvl_x + 35.0, row2_y + 20.0), 24.0, 1.0, PARCHMENT);

    // Stats/Upgrade Button
    let btn_rect = STATS_BTN_RECT;
    let is_hover = btn_rect.check_collision_point_rec(d.get_mouse_position());

    let mut color = Color::GRAY;
    if stats.stat_points > 0 {
        let alpha = ((d.get_time() * 5.0).sin() * 0.3 + 0.7) as f32;
        color = NEU_YELLOW.alpha(alpha);
    } else if is_hover {
        color = Color::WHITE;
    }

    d.draw_rectangle_lines_ex(btn_rect, 2.0, color);
    d.draw_text_ex(&assets.font_main, "UPGRADE STATS", Vector2::new(btn_rect.x + 40.0, btn_rect.y + 15.0), 20.0, 1.0, color);

    if stats.stat_points > 0 {
        d.draw_circle((btn_rect.x + btn_rect.width - 15.0) as i32, (btn_rect.y + 15.0) as i32, 8.0, NEU_RED);
        d.draw_text_ex(&assets.font_main, "!", Vector2::new(btn_rect.x + btn_rect.width - 18.0, btn_rect.y + 8.0), 14.0, 1.0, Color::WHITE);
    }
}

pub fn draw_sort_buttons(d: &mut RaylibDrawHandle, assets: &GameAssets) {
    let rank_rect = Rectangle::new(SORT_RANK_POS.x, SORT_RANK_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let suit_rect = Rectangle::new(SORT_SUIT_POS.x, SORT_SUIT_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);

    d.draw_rectangle_rounded(rank_rect, 0.2, 4, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(rank_rect, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text_ex(&assets.font_main, "Rank", Vector2::new(rank_rect.x + 40.0, rank_rect.y + 10.0), 24.0, 1.0, Color::BLACK);

    d.draw_rectangle_rounded(suit_rect, 0.2, 4, NEU_BLUE);
    d.draw_rectangle_rounded_lines_ex(suit_rect, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text_ex(&assets.font_main, "Suit", Vector2::new(suit_rect.x + 40.0, suit_rect.y + 10.0), 24.0, 1.0, Color::BLACK);
}

pub fn draw_relics(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    for (i, relic) in stats.equipped_relics.iter().enumerate() {
        let x = RELIC_START_X + (i as f32 * RELIC_SPACING);
        let y = RELIC_START_Y;
        let rect = Rectangle::new(x, y, RELIC_SIZE, RELIC_SIZE);
        d.draw_rectangle_rounded(rect, 0.2, 4, NEU_BLUE);
        d.draw_rectangle_rounded_lines_ex(rect, 0.2, 4, 2.0, NEU_ORANGE);
        let letter = &relic.name[0..1];
        // FIX: Use Custom Font
        d.draw_text_ex(&assets.font_main, letter, Vector2::new(x + 20.0, y + 15.0), 30.0, 1.0, PARCHMENT);
    }
}

pub fn draw_single_card(d: &mut RaylibDrawHandle, card: &Card, assets: &GameAssets) {
    // 1. Draw a massive debug crosshair at 0,0 to see if camera is centered
    d.draw_line(0, 0, 1000, 1000, Color::GREEN);

    // 2. DEBUG: Draw a Red Box at the card's position (Fallback)
    let debug_rect = Rectangle::new(
        card.current_pos.x, 
        card.current_pos.y, 
        CARD_WIDTH * card.scale.x, 
        CARD_HEIGHT * card.scale.y
    );
    d.draw_rectangle_rec(debug_rect, Color::RED); // <--- LOOK FOR THIS

    // 3. Normal Texture Drawing
    const SHEET_W: f32 = 5928.0;
    const SHEET_H: f32 = 2848.0;
    const COLS: f32 = 13.0;
    const ROWS: f32 = 4.0;
    let src_w = SHEET_W / COLS;
    let src_h = SHEET_H / ROWS;

    let col_idx = if card.value == 14 { 0 } else { card.value - 1 };
    let row_idx = match card.suit { 0 => 0, 1 => 1, 2 => 3, 3 => 2, _ => 0 };
    
    // DEBUG PRINT
    if card.value == 14 && card.suit == 0 { // Print only once for Ace of Spades
         println!("Src Rect: x={} y={} w={} h={}", col_idx as f32 * src_w, row_idx as f32 * src_h, src_w, src_h);
    }

    let source_rec = Rectangle::new(col_idx as f32 * src_w, row_idx as f32 * src_h, src_w, src_h);
    let dest_rect = Rectangle::new(card.current_pos.x, card.current_pos.y, CARD_WIDTH * card.scale.x, CARD_HEIGHT * card.scale.y);
    let origin = Vector2::new(dest_rect.width / 2.0, dest_rect.height / 2.0); // CORRECTED LINE
    let tint = if card.is_hovered { Color::WHITE } else { Color::new(245, 245, 245, 255) };
    
    d.draw_texture_pro(&assets.tex_spritesheet, source_rec, dest_rect, origin, card.rotation * 57.29, tint);
}

pub fn draw_enemy_monitor(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    let w = 260.0;
    let h = 160.0;
    let x = ENEMY_CENTER_X - w / 2.0;
    let y = ENEMY_Y;

    // Damage Flash Effect
    let scale_mod = if stats.damage_flash_timer > 0.0 { 4.0 } else { 0.0 };
    let final_rect = Rectangle::new(x - scale_mod, y - scale_mod, w + scale_mod*2.0, h + scale_mod*2.0);

    d.draw_rectangle_rounded(final_rect, 0.1, 4, Color::BLACK);
    d.draw_rectangle_rounded_lines_ex(final_rect, 0.1, 4, 6.0, Color::DARKGRAY);

    let screen_rect = Rectangle::new(x + 10.0, y + 10.0, w - 20.0, h - 20.0);
    d.draw_rectangle_rounded(screen_rect, 0.05, 4, Color::new(20, 30, 40, 255));

    // FONT UPDATE
    d.draw_text_ex(&assets.font_main, &stats.enemy_name, Vector2::new(x + 20.0, y + 25.0), 30.0, 1.0, NEU_RED);

    let ability_text = match stats.active_ability {
        BossAbility::SilenceSuit(_) => "SILENCE",
        BossAbility::HandSizeMinusOne => "HAND -1",
        BossAbility::DoubleTarget => "HP x2",
        BossAbility::PayToDiscard => "TAX $1",
        _ => "",
    };
    if !ability_text.is_empty() {
        d.draw_text_ex(&assets.font_main, ability_text, Vector2::new(x + 20.0, y + 60.0), 20.0, 1.0, Color::RED);
    }

    // HP Bar
    let current_score = stats.display_score as i32;
    let remaining_hp = (stats.target_score - current_score).max(0);
    let hp_pct = (remaining_hp as f32 / stats.target_score as f32).clamp(0.0, 1.0);

    let bar_w = w - 40.0;
    let bar_h = 24.0;
    let bar_x = x + 20.0;
    let bar_y = y + h - 45.0;

    d.draw_rectangle(bar_x as i32, bar_y as i32, bar_w as i32, bar_h as i32, Color::BLACK);

    let fill_color = if stats.damage_flash_timer > 0.0 {
        if stats.damage_flash_timer > 0.1 { Color::WHITE } else { NEU_RED }
    } else {
        NEU_BLUE
    };

    d.draw_rectangle(bar_x as i32, bar_y as i32, (bar_w * hp_pct) as i32, bar_h as i32, fill_color);
    d.draw_rectangle_lines(bar_x as i32, bar_y as i32, bar_w as i32, bar_h as i32, Color::GRAY);

    let hp_text = format!("{} / {}", remaining_hp, stats.target_score);
    let text_w = d.measure_text(&hp_text, 20);

    // FONT UPDATE
    d.draw_text_ex(&assets.font_main, &hp_text, Vector2::new(bar_x + bar_w/2.0 - text_w as f32/2.0, bar_y + 2.0), 20.0, 1.0, Color::WHITE);
}
