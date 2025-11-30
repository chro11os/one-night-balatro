use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::card::Card;
use crate::structures::assets::GameAssets;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;

pub fn draw_scene(d: &mut RaylibDrawHandle, stats: &BaseModifiers, hand: &[Card], state: &GameState, assets: &GameAssets, anim: &AnimationState) {
    match state {
        GameState::Playing => {
            d.clear_background(Color::new(45, 42, 40, 255));
            for y in (0..SCREEN_HEIGHT as i32).step_by(4) {
                d.draw_line(0, y, SCREEN_WIDTH as i32, y, Color::BLACK.alpha(0.1));
            }
            draw_sidebar(d, stats, anim, assets);
            draw_game_area(d, hand, assets, stats);

            for ft in &stats.floating_texts {
                let alpha = (ft.life / ft.max_life).clamp(0.0, 1.0);
                let color = ft.color.alpha(alpha);
                d.draw_text(&ft.text, (ft.pos.x + 2.0) as i32, (ft.pos.y + 2.0) as i32, ft.size, Color::BLACK.alpha(alpha));
                d.draw_text(&ft.text, ft.pos.x as i32, ft.pos.y as i32, ft.size, color);
            }

            if d.is_key_down(KeyboardKey::KEY_TAB) {
                draw_dev_toolbox(d);
            }
        },
        GameState::Shop => {
            draw_shop(d, stats);
            draw_stats_button(d, stats, assets);
        },
        GameState::StatsMenu => {
            d.clear_background(Color::new(45, 42, 40, 255));
            draw_sidebar(d, stats, anim, assets);
            d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, NEU_BLACK.alpha(0.8));
            draw_stats_menu(d, stats);
        }
        _ => d.clear_background(NEU_BG),
    }
    d.draw_fps(10, 10);
}

fn draw_sidebar(d: &mut RaylibDrawHandle, stats: &BaseModifiers, anim: &AnimationState, assets: &GameAssets) {
    let sb_w = SIDEBAR_WIDTH;
    let h = SCREEN_HEIGHT;
    d.draw_rectangle(0, 0, sb_w as i32, h as i32, NEU_BLACK);
    d.draw_line_ex(Vector2::new(sb_w, 0.0), Vector2::new(sb_w, h), 4.0, Color::BLACK);
    let pad = 10.0;

    // HP
    let hp_y = pad + 10.0;
    d.draw_text("PLAYER HP", (pad + 5.0) as i32, hp_y as i32, 20, PARCHMENT);
    let hp_bar_w = sb_w - pad * 2.0;
    let hp_rect_bg = Rectangle::new(pad, hp_y + 25.0, hp_bar_w, 30.0);
    d.draw_rectangle_rounded(hp_rect_bg, 0.2, 4, Color::new(30, 20, 20, 255));
    let hp_pct = stats.current_hp as f32 / stats.max_hp as f32;
    let hp_rect_fg = Rectangle::new(pad, hp_y + 25.0, hp_bar_w * hp_pct, 30.0);
    d.draw_rectangle_rounded(hp_rect_fg, 0.2, 4, NEU_RED);
    d.draw_text(&format!("{}/{}", stats.current_hp, stats.max_hp), (pad + 10.0) as i32, (hp_y + 30.0) as i32, 20, PARCHMENT);

    // --- ENEMY INFO (Updated with Texture) ---
    let enemy_y = hp_y + 70.0;
    let blind_rect = Rectangle::new(pad, enemy_y, sb_w - pad*2.0, 120.0);

    // Draw Blue Panel Texture
    d.draw_texture_pro(
        &assets.tex_panel_blue,
        Rectangle::new(0.0, 0.0, assets.tex_panel_blue.width as f32, assets.tex_panel_blue.height as f32),
        blind_rect,
        Vector2::zero(),
        0.0,
        Color::WHITE
    );

    // Text overlays
    d.draw_text(&stats.enemy_name, (pad + 15.0) as i32, (enemy_y + 10.0) as i32, 24, PARCHMENT);
    d.draw_text(&format!("Lvl {}", stats.level), (pad + 15.0) as i32, (enemy_y + 40.0) as i32, 20, NEU_YELLOW);

    // XP Bar (Black BG + Yellow FG)
    let xp_bar_w = 120.0;
    let xp_pct = stats.xp as f32 / stats.xp_target as f32;
    d.draw_rectangle((pad + 100.0) as i32, (enemy_y + 45.0) as i32, xp_bar_w as i32, 10, Color::BLACK);
    d.draw_rectangle((pad + 100.0) as i32, (enemy_y + 45.0) as i32, (xp_bar_w * xp_pct) as i32, 10, NEU_YELLOW);

    d.draw_text("Score to Kill:", (pad + 15.0) as i32, (enemy_y + 70.0) as i32, 16, PARCHMENT);
    d.draw_text(&format!("{}", stats.target_score), (pad + 15.0) as i32, (enemy_y + 90.0) as i32, 30, NEU_RED);

    // Round Score
    let round_y = enemy_y + 130.0;
    let round_rect = Rectangle::new(pad, round_y, sb_w - pad*2.0, 70.0);
    d.draw_rectangle_rounded(round_rect, 0.1, 4, NEU_BLACK);
    d.draw_rectangle_rounded_lines_ex(round_rect, 0.1, 4, 2.0, Color::GRAY);
    d.draw_text("Damage Dealt", (pad + 20.0) as i32, (round_y + 10.0) as i32, 16, Color::GRAY);
    let mut score_color = PARCHMENT;
    let mut score_size = 40;
    if stats.is_crit_active && *anim == AnimationState::Scoring {
        score_color = NEU_YELLOW;
        score_size = 45;
    } else if *anim == AnimationState::Scoring {
        score_color = NEU_ORANGE;
    }
    d.draw_text(&format!("{}", stats.display_score as i32), (pad + 20.0) as i32, (round_y + 25.0) as i32, score_size, score_color);

    // Hand Calc
    let score_y = round_y + 80.0;
    let score_rect = Rectangle::new(pad, score_y, sb_w - pad*2.0, 150.0);
    d.draw_rectangle_rounded(score_rect, 0.1, 4, Color::new(30, 30, 30, 255));
    d.draw_rectangle_rounded_lines_ex(score_rect, 0.1, 4, 2.0, Color::GRAY);
    let hand_name = match stats.hand_rank { Some(r) => format!("{:?}", r), None => "Choose Cards".to_string() };
    d.draw_text(&hand_name, (pad + 20.0) as i32, (score_y + 15.0) as i32, 24, PARCHMENT);
    let calc_box_y = score_y + 50.0;
    let box_w = 80.0;
    let box_h = 60.0;
    let chips_dest = Rectangle::new(pad + 10.0, calc_box_y, box_w, box_h);
    d.draw_texture_pro(&assets.tex_banner, Rectangle::new(0.0, 0.0, assets.tex_banner.width as f32, assets.tex_banner.height as f32), chips_dest, Vector2::zero(), 0.0, NEU_BLUE);
    d.draw_text(&format!("{}", stats.chips), (pad + 20.0) as i32, (calc_box_y + 15.0) as i32, 30, PARCHMENT);
    d.draw_text("X", (pad + 100.0) as i32, (calc_box_y + 15.0) as i32, 30, NEU_RED);
    let mult_dest = Rectangle::new(pad + 130.0, calc_box_y, box_w, box_h);
    d.draw_texture_pro(&assets.tex_banner, Rectangle::new(0.0, 0.0, assets.tex_banner.width as f32, assets.tex_banner.height as f32), mult_dest, Vector2::zero(), 0.0, NEU_RED);
    d.draw_text(&format!("{}", stats.mult), (pad + 145.0) as i32, (calc_box_y + 15.0) as i32, 30, PARCHMENT);

    // Stats
    let stats_y = 500.0;
    draw_stat_box(d, "Hands", stats.hands_remaining, NEU_BLUE, 45.0, stats_y);
    draw_stat_box(d, "Discards", stats.discards_remaining, NEU_RED, 155.0, stats_y);
    let money_y = stats_y + 80.0;
    let money_rect = Rectangle::new(90.0, money_y, 120.0, 60.0);
    d.draw_rectangle_rounded(money_rect, 0.2, 4, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(money_rect, 0.2, 4, 3.0, Color::BLACK);
    let money_text = format!("$ {}", stats.money);
    let money_w = d.measure_text(&money_text, 30);
    let money_text_x = 90.0 + (120.0 - money_w as f32) / 2.0;
    d.draw_text(&money_text, money_text_x as i32, (money_y + 15.0) as i32, 30, Color::BLACK);
}

// ... [draw_stats_menu, draw_stats_button, draw_shop, draw_stat_box, draw_dev_toolbox, draw_game_area, draw_single_card] ...
// (Unchanged. Paste from previous steps to complete the file.)

fn draw_stats_menu(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    let center_x = SCREEN_WIDTH/2.0 - STAT_WIN_W/2.0;
    let center_y = SCREEN_HEIGHT/2.0 - STAT_WIN_H/2.0;
    let rect = Rectangle::new(center_x, center_y, STAT_WIN_W, STAT_WIN_H);
    d.draw_rectangle_rounded(rect, 0.1, 10, NEU_BLACK);
    d.draw_rectangle_rounded_lines_ex(rect, 0.1, 10, 3.0, NEU_ORANGE);
    let title = "RPG STATS";
    let tw = d.measure_text(title, 40);
    d.draw_text(title, (center_x + STAT_WIN_W/2.0 - tw as f32/2.0) as i32, (center_y + 20.0) as i32, 40, NEU_ORANGE);
    let pts_text = format!("Points Available: {}", stats.stat_points);
    let pts_w = d.measure_text(&pts_text, 20);
    d.draw_text(&pts_text, (center_x + STAT_WIN_W/2.0 - pts_w as f32/2.0) as i32, (center_y + 60.0) as i32, 20, PARCHMENT);
    let x_label = center_x + 60.0;
    let x_val = center_x + 250.0;
    let x_btn = center_x + 350.0;
    let mut draw_stat_row = |y_pos: f32, label: &str, val: String| {
        d.draw_text(label, x_label as i32, (y_pos + 5.0) as i32, 20, PARCHMENT);
        d.draw_text(&val, x_val as i32, (y_pos + 5.0) as i32, 20, NEU_BLUE);
        if stats.stat_points > 0 {
            let btn_rect = Rectangle::new(x_btn, y_pos, 30.0, 30.0);
            d.draw_rectangle_rounded(btn_rect, 0.2, 4, NEU_ORANGE);
            d.draw_text("+", (x_btn + 8.0) as i32, (y_pos + 2.0) as i32, 24, NEU_BLACK);
        } else {
            let btn_rect = Rectangle::new(x_btn, y_pos, 30.0, 30.0);
            d.draw_rectangle_rounded(btn_rect, 0.2, 4, Color::DARKGRAY);
        }
    };
    draw_stat_row(center_y + 100.0, "Max HP", format!("{}", stats.max_hp));
    draw_stat_row(center_y + 150.0, "Crit Chance", format!("{:.0}%", stats.crit_chance * 100.0));
    draw_stat_row(center_y + 200.0, "Crit Dmg", format!("{:.1}x", stats.crit_mult));
    d.draw_text(&format!("Current Level: {}", stats.level), (center_x + 60.0) as i32, (center_y + 260.0) as i32, 20, Color::GRAY);
    d.draw_text(&format!("XP: {} / {}", stats.xp, stats.xp_target), (center_x + 60.0) as i32, (center_y + 290.0) as i32, 20, Color::GRAY);
    let back_rect = Rectangle::new(center_x + 150.0, center_y + STAT_WIN_H - 60.0, 200.0, 40.0);
    d.draw_rectangle_rounded(back_rect, 0.2, 4, NEU_BLUE);
    d.draw_text("BACK", (back_rect.x + 70.0) as i32, (back_rect.y + 10.0) as i32, 20, PARCHMENT);
}

fn draw_stats_button(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    let rect = Rectangle::new(STATS_BTN_X, STATS_BTN_Y, STATS_BTN_W, STATS_BTN_H);
    let texture = if stats.stat_points > 0 { &assets.tex_btn_plus_active } else { &assets.tex_btn_plus_disabled };
    d.draw_texture_pro(texture, Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32), rect, Vector2::zero(), 0.0, Color::WHITE);
    d.draw_text("+", (rect.x + 12.0) as i32, (rect.y + 8.0) as i32, 30, Color::WHITE);
}

fn draw_shop(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    d.clear_background(NEU_BLACK);
    let title = "SHOP / REST AREA";
    let w = d.measure_text(title, 60);
    d.draw_text(title, (SCREEN_WIDTH as i32 - w)/2, 100, 60, NEU_ORANGE);
    let info = format!("HP: {}/{}  |  Money: ${}", stats.current_hp, stats.max_hp, stats.money);
    let info_w = d.measure_text(&info, 30);
    d.draw_text(&info, (SCREEN_WIDTH as i32 - info_w)/2, 200, 30, PARCHMENT);
    let btn_rect = Rectangle::new(SCREEN_WIDTH/2.0 - 100.0, SCREEN_HEIGHT/2.0 + 100.0, 200.0, 60.0);
    d.draw_rectangle_rounded(btn_rect, 0.2, 4, NEU_BLUE);
    d.draw_text("NEXT FIGHT", (btn_rect.x + 30.0) as i32, (btn_rect.y + 20.0) as i32, 20, PARCHMENT);
}

fn draw_stat_box(d: &mut RaylibDrawHandle, label: &str, val: i32, color: Color, x: f32, y: f32) {
    let rect = Rectangle::new(x, y, 100.0, 70.0);
    d.draw_rectangle_rounded(rect, 0.1, 4, NEU_BLACK);
    d.draw_rectangle_rounded_lines_ex(rect, 0.1, 4, 2.0, color);
    d.draw_text(label, (x + 15.0) as i32, (y + 10.0) as i32, 16, color);
    d.draw_text(&format!("{}", val), (x + 40.0) as i32, (y + 35.0) as i32, 30, PARCHMENT);
}

fn draw_dev_toolbox(d: &mut RaylibDrawHandle) {
    let box_h = (DEV_BTN_H + DEV_GAP) * 6.0;
    d.draw_rectangle_rounded(Rectangle::new(DEV_BOX_X, DEV_BOX_Y, DEV_BTN_W + 20.0, box_h), 0.1, 10, NEU_BLACK.alpha(0.9));
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(DEV_BOX_X, DEV_BOX_Y, DEV_BTN_W + 20.0, box_h), 0.1, 10, 2.0, NEU_ORANGE);
    d.draw_text("DEV TOOLS", (DEV_BOX_X + 20.0) as i32, (DEV_BOX_Y + 10.0) as i32, 20, NEU_ORANGE);
    let start_x = DEV_BOX_X + 10.0;
    let mut start_y = DEV_BOX_Y + 40.0;
    let labels = ["+ Discard", "+ Hand", "+ XP (Test)", "+ Money", "Insta-Win"];
    for label in labels.iter() {
        let rect = Rectangle::new(start_x, start_y, DEV_BTN_W, DEV_BTN_H);
        let color = if rect.check_collision_point_rec(d.get_mouse_position()) { NEU_BLUE } else { Color::GRAY };
        d.draw_rectangle_rounded(rect, 0.2, 4, color);
        d.draw_text(label, (rect.x + 10.0) as i32, (rect.y + 10.0) as i32, 20, PARCHMENT);
        start_y += DEV_BTN_H + DEV_GAP;
    }
}

fn draw_game_area(d: &mut RaylibDrawHandle, hand: &[Card], assets: &GameAssets, stats: &BaseModifiers) {
    let center_x = SIDEBAR_WIDTH + (SCREEN_WIDTH - SIDEBAR_WIDTH) / 2.0;
    let btn_y = 660.0;
    let sort_y = 620.0;
    let sort_rank_rect = Rectangle::new(center_x - SORT_BTN_WIDTH - 5.0, sort_y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    d.draw_rectangle_rounded(sort_rank_rect, 0.2, 4, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(sort_rank_rect, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text("Rank", (sort_rank_rect.x + 20.0) as i32, (sort_rank_rect.y + 8.0) as i32, 16, Color::BLACK);
    let sort_suit_rect = Rectangle::new(center_x + 5.0, sort_y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    d.draw_rectangle_rounded(sort_suit_rect, 0.2, 4, NEU_BLUE);
    d.draw_rectangle_rounded_lines_ex(sort_suit_rect, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text("Suit", (sort_suit_rect.x + 25.0) as i32, (sort_suit_rect.y + 8.0) as i32, 16, PARCHMENT);
    let play_rect = Rectangle::new(center_x - BTN_WIDTH - 10.0, btn_y, BTN_WIDTH, BTN_HEIGHT);
    d.draw_texture_pro(&assets.tex_btn_play, Rectangle::new(0.0, 0.0, assets.tex_btn_play.width as f32, assets.tex_btn_play.height as f32), play_rect, Vector2::zero(), 0.0, Color::WHITE);
    d.draw_text("PLAY", (play_rect.x + 35.0) as i32, (play_rect.y + 12.0) as i32, 20, Color::WHITE);
    let disc_rect = Rectangle::new(center_x + 10.0, btn_y, BTN_WIDTH, BTN_HEIGHT);
    d.draw_texture_pro(&assets.tex_btn_discard, Rectangle::new(0.0, 0.0, assets.tex_btn_discard.width as f32, assets.tex_btn_discard.height as f32), disc_rect, Vector2::zero(), 0.0, Color::WHITE);
    d.draw_text("DISCARD", (disc_rect.x + 15.0) as i32, (disc_rect.y + 12.0) as i32, 20, Color::WHITE);

    draw_stats_button(d, stats, assets);

    for card in hand { draw_single_card(d, card, assets); }
}

fn draw_single_card(d: &mut RaylibDrawHandle, card: &Card, assets: &GameAssets) {
    let width = CARD_WIDTH * card.scale;
    let height = CARD_HEIGHT * card.scale;
    let dest_rect = Rectangle::new(card.current_pos.x, card.current_pos.y, width, height);
    let suit_str = match card.suit { 0 => "clubs", 1 => "diamonds", 2 => "hearts", 3 => "spades", _ => "" };
    let rank_str = match card.value { 11 => "J".to_string(), 12 => "Q".to_string(), 13 => "K".to_string(), 14 => "A".to_string(), v => format!("{:02}", v) };
    let key = format!("card_{}_{}", suit_str, rank_str);
    if let Some(texture) = assets.tex_cards.get(&key) {
        let origin = Vector2::new(width / 2.0, height / 2.0);
        d.draw_texture_pro(texture, Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32), Rectangle::new(dest_rect.x + 6.0, dest_rect.y + 6.0, width, height), origin, card.rotation * 57.29, Color::BLACK.alpha(0.6));
        let tint = if card.is_hovered { Color::WHITE } else {
            match card.suit {
                0 => TINT_CLUBS,
                1 => TINT_DIAMONDS,
                2 => TINT_HEARTS,
                _ => TINT_SPADES,
            }
        };
        d.draw_texture_pro(texture, Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32), Rectangle::new(dest_rect.x, dest_rect.y, width, height), origin, card.rotation * 57.29, tint);
    }
}