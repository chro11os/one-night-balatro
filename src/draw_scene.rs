use raylib::prelude::*;
use crate::structures::stats::{BaseModifiers, BossAbility, RuneType};
use crate::structures::card::Card;
use crate::structures::assets::GameAssets;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;

pub fn draw_scene(d: &mut RaylibDrawHandle, stats: &BaseModifiers, hand: &[Card], state: &GameState, assets: &GameAssets, anim: &AnimationState) {
    let camera = Camera2D {
        target: Vector2::new(0.0, 0.0),
        offset: stats.screen_shake,
        rotation: 0.0,
        zoom: 1.0,
    };

    match state {
        GameState::Playing => {
            let src = Rectangle::new(0.0, 0.0, assets.tex_background.width as f32, assets.tex_background.height as f32);
            let dest = Rectangle::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT);
            d.draw_texture_pro(&assets.tex_background, src, dest, Vector2::zero(), 0.0, Color::WHITE);

            {
                let mut d_cam = d.begin_mode2D(camera);
                draw_enemy_monitor(&mut d_cam, stats, assets);
                draw_player_panel(&mut d_cam, stats, assets);
                draw_action_panel(&mut d_cam, stats, anim, assets, hand);
                draw_sort_buttons(&mut d_cam, assets);
                draw_relics(&mut d_cam, stats);
                draw_game_area(&mut d_cam, hand, assets, stats);

                for ft in &stats.floating_texts {
                    let alpha = (ft.life / ft.max_life).clamp(0.0, 1.0);
                    let color = ft.color.alpha(alpha);
                    d_cam.draw_text(&ft.text, (ft.pos.x + 2.0) as i32, (ft.pos.y + 2.0) as i32, ft.size, Color::BLACK.alpha(alpha));
                    d_cam.draw_text(&ft.text, ft.pos.x as i32, ft.pos.y as i32, ft.size, color);
                }

                for p in &stats.particles {
                    let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
                    let color = p.color.alpha(alpha);
                    let rec = Rectangle::new(p.pos.x, p.pos.y, p.size, p.size);
                    let origin = Vector2::new(p.size/2.0, p.size/2.0);
                    d_cam.draw_rectangle_pro(rec, origin, p.rotation * 57.29, color);
                }
            }
        },
        GameState::RuneSelect => {
            d.clear_background(NEU_BG);
            // FIX: Pass assets to draw_rune_select
            draw_rune_select(d, stats, assets);
        },
        GameState::Shop => {
            draw_shop(d, stats);
        },
        GameState::StatsMenu => {
            {
                let mut d_cam = d.begin_mode2D(camera);
                draw_player_panel(&mut d_cam, stats, assets);
            }
            d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, NEU_BLACK.alpha(0.85));
            draw_stats_menu(d, stats);
        },
        GameState::BattleResult => {
            d.clear_background(NEU_BLACK);
            draw_battle_result(d, stats);
        },
        GameState::Menu => {
            d.clear_background(NEU_BG);
            draw_menu(d);
        },
        _ => d.clear_background(NEU_BG),
    }
    d.draw_fps(10, 10);
}

fn draw_game_area(d: &mut RaylibDrawHandle, hand: &[Card], assets: &GameAssets, _stats: &BaseModifiers) {
    // 1. Draw non-selected (Hand) cards first (Bottom Layer)
    for card in hand.iter().filter(|c| !c.is_selected) {
        draw_single_card(d, card, assets);
    }
    // 2. Draw selected (Scoring/Moving) cards last (Top Layer)
    for card in hand.iter().filter(|c| c.is_selected) {
        draw_single_card(d, card, assets);
    }
}

fn draw_action_panel(d: &mut RaylibDrawHandle, stats: &BaseModifiers, _anim: &AnimationState, assets: &GameAssets, hand: &[Card]) {
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

    d.draw_text_ex(&assets.font_main, &format!("Hands: {}", stats.hands_remaining), Vector2::new(play_rect.x + 10.0, play_rect.y + 65.0), 18.0, 1.0, NEU_BLUE);
    d.draw_text_ex(&assets.font_main, &format!("Disc: {}", stats.discards_remaining), Vector2::new(disc_rect.x + 10.0, disc_rect.y + 65.0), 18.0, 1.0, NEU_RED);

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
    d.draw_text_ex(&assets.font_main, &format!("{}", stats.chips), Vector2::new(chips_x + 15.0, chips_y + 20.0), 34.0, 1.0, PARCHMENT);
    d.draw_text_ex(&assets.font_main, "X", Vector2::new(chips_x + box_w + 10.0, chips_y + 20.0), 34.0, 1.0, Color::WHITE);

    let mult_x = chips_x + box_w + 40.0;
    d.draw_texture_pro(&assets.tex_banner, Rectangle::new(0.0,0.0, assets.tex_banner.width as f32, assets.tex_banner.height as f32), Rectangle::new(mult_x, chips_y, box_w, box_h), Vector2::zero(), 0.0, NEU_RED);

    // FONT UPDATE
    d.draw_text_ex(&assets.font_main, &format!("{}", stats.mult), Vector2::new(mult_x + 15.0, chips_y + 20.0), 34.0, 1.0, PARCHMENT);
}

fn draw_player_panel(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
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
    d.draw_text_ex(&assets.font_main, &format!("{}/{}", stats.current_hp, stats.max_hp), Vector2::new(x + hp_margin + 10.0, hp_y + 5.0), 20.0, 1.0, Color::WHITE);

    let row2_y = hp_y + hp_bar_h + 20.0;
    let box_w = (hp_w - 10.0) / 2.0;
    let box_h = 50.0;
    d.draw_rectangle_lines((x + hp_margin) as i32, row2_y as i32, box_w as i32, box_h as i32, NEU_ORANGE);
    d.draw_text_ex(&assets.font_main, "MONEY", Vector2::new(x + hp_margin + 5.0, row2_y + 5.0), 16.0, 1.0, NEU_ORANGE);
    d.draw_text_ex(&assets.font_main, &format!("$ {}", stats.money), Vector2::new(x + hp_margin + 20.0, row2_y + 20.0), 24.0, 1.0, PARCHMENT);

    let lvl_x = x + hp_margin + box_w + 10.0;
    d.draw_rectangle_lines(lvl_x as i32, row2_y as i32, box_w as i32, box_h as i32, NEU_BLUE);
    d.draw_text_ex(&assets.font_main, "LEVEL", Vector2::new(lvl_x + 5.0, row2_y + 5.0), 16.0, 1.0, NEU_BLUE);
    d.draw_text_ex(&assets.font_main, &format!("{}", stats.level), Vector2::new(lvl_x + 35.0, row2_y + 20.0), 24.0, 1.0, PARCHMENT);

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

fn draw_rune_select(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    let center_x = SCREEN_WIDTH / 2.0;
    let content_offset = RUNE_CONTENT_OFFSET;
    let start_y_base = RUNE_START_Y;

    d.draw_text("CHOOSE YOUR PATH", (center_x + content_offset - 200.0) as i32, 50, 60, NEU_ORANGE);
    d.draw_text("Select one rune from each row.", (center_x + content_offset - 220.0) as i32, 120, 24, Color::GRAY);

    let rows = [
        (RuneType::Red, "COMBAT STYLE", NEU_RED, start_y_base),
        (RuneType::Blue, "UTILITY", NEU_BLUE, start_y_base + RUNE_SPACING_Y),
        (RuneType::Green, "ECONOMY", Color::new(76, 175, 80, 255), start_y_base + RUNE_SPACING_Y * 2.0),
        (RuneType::Minor, "STARTING BONUS", PARCHMENT, start_y_base + RUNE_SPACING_Y * 3.0),
    ];

    let mouse_pos = d.get_mouse_position();
    let mut hovered_rune_name = String::new();
    let mut hovered_rune_desc = String::new();
    let mut hovered_rune_color = Color::WHITE;

    for (r_type, label, color, y_pos) in rows.iter() {
        d.draw_text(label, 50, (*y_pos - 10.0) as i32, 20, *color);

        let row_runes: Vec<_> = stats.available_runes.iter().filter(|r| r.rune_type == *r_type).collect();
        let count = row_runes.len();
        if count == 0 { continue; }

        let spacing = RUNE_SPACING_X;
        let row_width = (count as f32 - 1.0) * spacing;
        let start_x = (center_x + content_offset) - row_width / 2.0;

        for (i, rune) in row_runes.iter().enumerate() {
            let cx = start_x + (i as f32 * spacing);
            let cy = *y_pos;
            let radius = RUNE_RADIUS;

            let is_equipped = stats.equipped_runes.iter().any(|r| r.id == rune.id);
            let is_hovered = ((mouse_pos.x - cx).powi(2) + (mouse_pos.y - cy).powi(2)).sqrt() < radius;

            // 1. Draw Selection Halo
            if is_equipped {
                d.draw_circle_lines(cx as i32, cy as i32, radius + 4.0, NEU_ORANGE);
                d.draw_circle(cx as i32, cy as i32, radius + 2.0, color.alpha(0.2));
            } else if is_hovered {
                d.draw_circle_lines(cx as i32, cy as i32, radius + 2.0, Color::WHITE);
            } else {
                d.draw_circle_lines(cx as i32, cy as i32, radius, color.alpha(0.5));
            }

            // 2. Draw Icon (if available) OR Fallback Letter
            if let Some(tex) = assets.rune_icons.get(&rune.name) {
                // Scale texture to fit inside radius (64x64 roughly)
                let icon_size = radius * 2.0;
                let dest_rect = Rectangle::new(cx - radius, cy - radius, icon_size, icon_size);
                let src_rect = Rectangle::new(0.0, 0.0, tex.width as f32, tex.height as f32);

                // Tint gray if not selected/hovered
                let tint = if is_equipped || is_hovered { Color::WHITE } else { Color::GRAY };

                d.draw_texture_pro(tex, src_rect, dest_rect, Vector2::zero(), 0.0, tint);
            } else {
                // Fallback: Circle Background + Letter
                d.draw_circle(cx as i32, cy as i32, radius, color.alpha(0.2));
                let letter = &rune.name[0..1];
                d.draw_text(letter, (cx - 10.0) as i32, (cy - 15.0) as i32, 30, PARCHMENT);
            }

            if is_hovered {
                hovered_rune_name = rune.name.clone();
                hovered_rune_desc = rune.description.clone();
                hovered_rune_color = *color;
            }
        }
    }

    if !hovered_rune_name.is_empty() {
        let info_x = SCREEN_WIDTH / 2.0 - 60.0;
        let info_y = 250.0;
        let info_w = 320.0;
        let info_h = 250.0;

        d.draw_rectangle_rounded(Rectangle::new(info_x, info_y, info_w, info_h), 0.1, 4, NEU_BLACK.alpha(0.9));
        d.draw_rectangle_rounded_lines_ex(Rectangle::new(info_x, info_y, info_w, info_h), 0.1, 4, 2.0, hovered_rune_color);

        d.draw_text(&hovered_rune_name, (info_x + 20.0) as i32, (info_y + 20.0) as i32, 30, hovered_rune_color);
        d.draw_text("Effect:", (info_x + 20.0) as i32, (info_y + 60.0) as i32, 20, Color::GRAY);

        let max_text_width = info_w - 40.0;
        let font_size = 20;
        let mut current_y = info_y + 90.0;

        for paragraph in hovered_rune_desc.split('\n') {
            let words: Vec<&str> = paragraph.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                let test_line = if current_line.is_empty() { word.to_string() } else { format!("{} {}", current_line, word) };
                if d.measure_text(&test_line, font_size) as f32 > max_text_width {
                    d.draw_text(&current_line, (info_x + 20.0) as i32, current_y as i32, font_size, PARCHMENT);
                    current_line = word.to_string();
                    current_y += 24.0;
                } else {
                    current_line = test_line;
                }
            }
            if !current_line.is_empty() {
                d.draw_text(&current_line, (info_x + 20.0) as i32, current_y as i32, font_size, PARCHMENT);
                current_y += 24.0;
            }
        }
    }

    let panel_x = SCREEN_WIDTH - 420.0;
    let panel_w = 380.0;
    let panel_h = SCREEN_HEIGHT - 200.0;

    d.draw_rectangle_rounded(Rectangle::new(panel_x, 100.0, panel_w, panel_h), 0.05, 4, NEU_BLACK.alpha(0.8));
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(panel_x, 100.0, panel_w, panel_h), 0.05, 4, 2.0, NEU_ORANGE);

    d.draw_text("CURRENT LOADOUT", (panel_x + 80.0) as i32, 120, 24, PARCHMENT);

    let mut benefits: Vec<String> = Vec::new();
    let mut downsides: Vec<String> = Vec::new();

    for rune in &stats.equipped_runes {
        let name = &rune.name;
        match name.as_str() {
            "Paladin" => {
                benefits.push(format!("- {}: +40 Max HP", name));
                downsides.push(format!("- {}: -10% Relic Mult", name));
            },
            "Reaper" => {
                benefits.push(format!("- {}: Lifesteal on Kill", name));
                downsides.push(format!("- {}: -15 Max HP", name));
            },
            "Judgement" => {
                benefits.push(format!("- {}: Balanced Calc", name));
                downsides.push(format!("- {}: Enemies 2x HP", name));
            },
            "Midas" => {
                benefits.push(format!("- {}: Gain Gold on Win", name));
                downsides.push(format!("- {}: Lose Gold on Loss", name));
            },
            "Greed" => {
                benefits.push(format!("- {}: +1 Hand, +1 Disc", name));
                downsides.push(format!("- {}: -1 Relic Slot", name));
            },
            "Investment" => {
                benefits.push(format!("- {}: +5% Interest", name));
                downsides.push(format!("- {}: Start with $0", name));
            },
            "Merchant" => {
                benefits.push(format!("- {}: +1 Shop Slot", name));
                downsides.push(format!("- {}: Prices +20%", name));
            },
            "Mentalist" => {
                benefits.push(format!("- {}: Free Scrolls", name));
                downsides.push(format!("- {}: Less Scrolls", name));
            },
            "Evolution" => {
                benefits.push(format!("- {}: Relics 1.5x Val", name));
                downsides.push(format!("- {}: Enemies 1.5x HP", name));
            },
            "Force" => benefits.push(format!("- {}: +10 Base Mult", name)),
            "Flow" => benefits.push(format!("- {}: +10 Base Chips", name)),
            "Wealth" => benefits.push(format!("- {}: +3 Gold/Round", name)),
            _ => {}
        }
    }

    let mut list_y = 170.0;

    d.draw_text("BENEFITS", (panel_x + 20.0) as i32, list_y as i32, 20, Color::GRAY);
    list_y += 30.0;
    for text in benefits {
        d.draw_text(&text, (panel_x + 30.0) as i32, list_y as i32, 20, NEU_GREEN);
        list_y += 30.0;
    }

    list_y += 20.0;

    d.draw_text("TRADE-OFFS", (panel_x + 20.0) as i32, list_y as i32, 20, Color::GRAY);
    list_y += 30.0;
    for text in downsides {
        d.draw_text(&text, (panel_x + 30.0) as i32, list_y as i32, 20, NEU_RED);
        list_y += 30.0;
    }

    let btn_w = 250.0;
    let btn_h = 70.0;
    let btn_x = center_x + content_offset - btn_w / 2.0;
    let btn_y = SCREEN_HEIGHT - 120.0;
    let btn_rect = Rectangle::new(btn_x, btn_y, btn_w, btn_h);
    let (off, shad) = get_button_offset(d, btn_rect);

    d.draw_rectangle_rounded(Rectangle::new(btn_x, btn_y + shad, btn_w, btn_h), 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(Rectangle::new(btn_x, btn_y + off, btn_w, btn_h), 0.2, 4, NEU_ORANGE);
    d.draw_text("START RUN", (btn_x + 55.0) as i32, (btn_y + 20.0 + off) as i32, 28, Color::BLACK);
}

fn draw_stats_menu(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    let center_x = SCREEN_WIDTH / 2.0;
    let center_y = SCREEN_HEIGHT / 2.0;
    let box_w = 600.0;
    let box_h = 500.0;

    d.draw_rectangle_rounded(Rectangle::new(center_x - box_w/2.0, center_y - box_h/2.0, box_w, box_h), 0.05, 4, NEU_BLACK);
    d.draw_rectangle_rounded_lines_ex(Rectangle::new(center_x - box_w/2.0, center_y - box_h/2.0, box_w, box_h), 0.05, 4, 3.0, NEU_ORANGE);

    // Close Button
    let close_btn_x = center_x + box_w/2.0 - 45.0;
    let close_btn_y = center_y - box_h/2.0 + 15.0;
    d.draw_rectangle(close_btn_x as i32, close_btn_y as i32, 30, 30, NEU_RED);
    d.draw_text("X", (close_btn_x + 8.0) as i32, (close_btn_y + 5.0) as i32, 20, Color::WHITE);

    d.draw_text("UPGRADES", (center_x - 100.0) as i32, (center_y - box_h/2.0 + 30.0) as i32, 40, NEU_ORANGE);
    d.draw_text(&format!("Points Available: {}", stats.stat_points), (center_x - 120.0) as i32, (center_y - box_h/2.0 + 80.0) as i32, 30, PARCHMENT);

    let start_y = center_y - 100.0;
    let row_h = 60.0;
    let btn_w = 120.0;
    let btn_h = 40.0;

    let stats_list = [
        ("Max HP", format!("{}", stats.max_hp), "HP"),
        ("Crit Chance", format!("{:.0}%", stats.crit_chance * 100.0), "Luck"),
        ("Crit Dmg", format!("{:.1}x", stats.crit_mult), "Power"),
    ];

    for (i, (label, val, _desc)) in stats_list.iter().enumerate() {
        let y = start_y + (i as f32 * row_h);
        d.draw_text(label, (center_x - 200.0) as i32, y as i32, 30, Color::WHITE);
        d.draw_text(val, (center_x - 20.0) as i32, y as i32, 30, NEU_YELLOW);

        if stats.stat_points > 0 {
            let btn_rect = Rectangle::new(center_x + 100.0, y, btn_w, btn_h);
            d.draw_rectangle_rounded(btn_rect, 0.2, 4, NEU_GREEN);
            d.draw_text("+ UPGRADE", (btn_rect.x + 10.0) as i32, (btn_rect.y + 10.0) as i32, 20, Color::BLACK);
        }
    }
}

// Helpers
fn draw_sort_buttons(d: &mut RaylibDrawHandle, _assets: &GameAssets) {
    let rank_rect = Rectangle::new(SORT_RANK_POS.x, SORT_RANK_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let suit_rect = Rectangle::new(SORT_SUIT_POS.x, SORT_SUIT_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);

    d.draw_rectangle_rounded(rank_rect, 0.2, 4, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(rank_rect, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text("Rank", (rank_rect.x + 40.0) as i32, (rank_rect.y + 10.0) as i32, 24, Color::BLACK);

    d.draw_rectangle_rounded(suit_rect, 0.2, 4, NEU_BLUE);
    d.draw_rectangle_rounded_lines_ex(suit_rect, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text("Suit", (suit_rect.x + 40.0) as i32, (suit_rect.y + 10.0) as i32, 24, Color::BLACK);
}

fn get_button_offset(d: &RaylibDrawHandle, rect: Rectangle) -> (f32, f32) {
    let mouse_pos = d.get_mouse_position();
    let is_hovered = rect.check_collision_point_rec(mouse_pos);
    let is_down = d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

    if is_hovered {
        if is_down { return (4.0, 0.0); }
        else { return (-2.0, 6.0); }
    }
    (0.0, 3.0)
}

fn draw_relics(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    for (i, relic) in stats.equipped_relics.iter().enumerate() {
        let x = RELIC_START_X + (i as f32 * RELIC_SPACING);
        let y = RELIC_START_Y;
        let rect = Rectangle::new(x, y, RELIC_SIZE, RELIC_SIZE);
        d.draw_rectangle_rounded(rect, 0.2, 4, NEU_BLUE);
        d.draw_rectangle_rounded_lines_ex(rect, 0.2, 4, 2.0, NEU_ORANGE);
        let letter = &relic.name[0..1];
        // Note: For relics, we might not have 'assets' passed here in the original struct,
        // so we keep default text unless we change the signature to pass assets.
        // For now, let's assume we stick to default or pass assets if you update the call in draw_scene.
        d.draw_text(letter, (x + 20.0) as i32, (y + 15.0) as i32, 30, PARCHMENT);
    }
}

fn draw_single_card(d: &mut RaylibDrawHandle, card: &Card, assets: &GameAssets) {
    const SHEET_W: f32 = 5928.0;
    const SHEET_H: f32 = 2848.0;
    const COLS: f32 = 13.0;
    const ROWS: f32 = 4.0;
    let src_w = SHEET_W / COLS;
    let src_h = SHEET_H / ROWS;

    let col_idx = if card.value == 14 { 0 } else { card.value - 1 };
    let row_idx = match card.suit { 0 => 0, 1 => 1, 2 => 3, 3 => 2, _ => 0 };
    let source_rec = Rectangle::new(col_idx as f32 * src_w, row_idx as f32 * src_h, src_w, src_h);
    let dest_width = CARD_WIDTH * card.scale;
    let dest_height = CARD_HEIGHT * card.scale;
    let dest_rect = Rectangle::new(card.current_pos.x, card.current_pos.y, dest_width, dest_height);
    let origin = Vector2::new(dest_width / 2.0, dest_height / 2.0);
    let tint = if card.is_hovered { Color::WHITE } else { Color::new(245, 245, 245, 255) };
    d.draw_texture_pro(&assets.tex_spritesheet, source_rec, dest_rect, origin, card.rotation * 57.29, tint);
}

fn draw_shop(d: &mut RaylibDrawHandle, _stats: &BaseModifiers) { d.draw_text("Shop", 100, 100, 20, Color::WHITE); }
fn draw_battle_result(d: &mut RaylibDrawHandle, _stats: &BaseModifiers) { d.draw_text("Battle Result", 100, 100, 20, Color::WHITE); }
fn draw_dev_toolbox(_d: &mut RaylibDrawHandle) {}
fn draw_menu(d: &mut RaylibDrawHandle) {
    // Note: draw_menu doesn't have access to assets in the original signature.
    // You must update `draw_menu` signature in `draw_scene` to `draw_menu(d, assets)` to use the font here.
    // I will leave it as default for now to prevent compilation errors if the signature isn't updated.
    let text = "ONE NIGHT BALATRO";
    let font_size = 80;
    let text_w = d.measure_text(text, font_size);
    let x = (SCREEN_WIDTH as i32 - text_w) / 2;
    let y = (SCREEN_HEIGHT as i32 / 2) - 100;
    d.draw_text(text, x, y, font_size, PARCHMENT);

    let sub = "Click to Start";
    let sub_size = 40;
    let sub_w = d.measure_text(sub, sub_size);
    let sx = (SCREEN_WIDTH as i32 - sub_w) / 2;
    d.draw_text(sub, sx, y + 100, sub_size, Color::GRAY);
}
fn draw_enemy_monitor(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
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