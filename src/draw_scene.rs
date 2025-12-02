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
            // Draw Background directly (no clear_background needed if image covers screen)
            // Scaling the background to fit the 1280x720 window
            let src = Rectangle::new(0.0, 0.0, assets.tex_background.width as f32, assets.tex_background.height as f32);
            let dest = Rectangle::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT);
            d.draw_texture_pro(&assets.tex_background, src, dest, Vector2::zero(), 0.0, Color::WHITE);

            {
                let mut d_cam = d.begin_mode2D(camera);

                // Sidebar is still drawn for info, but we might want to make it semi-transparent or integrated later
                draw_sidebar(&mut d_cam, stats, anim, assets);

                // Draw Gameplay Elements
                draw_game_area(&mut d_cam, hand, assets, stats);
                draw_relics(&mut d_cam, stats);
                draw_dice(&mut d_cam);

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
            } // End Camera Mode

            if d.is_key_down(KeyboardKey::KEY_TAB) {
                draw_dev_toolbox(d);
            }
        },
        GameState::RuneSelect => {
            d.clear_background(NEU_BG);
            draw_rune_select(d, stats);
        },
        GameState::Shop => {
            draw_shop(d, stats);
        },
        GameState::StatsMenu => {
            d.clear_background(Color::new(45, 42, 40, 255));
            draw_sidebar(d, stats, anim, assets);
            d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, NEU_BLACK.alpha(0.8));
            draw_stats_menu(d, stats);
        },
        GameState::BattleResult => {
            d.clear_background(NEU_BLACK);
            draw_battle_result(d, stats);
        },
        _ => d.clear_background(NEU_BG),
    }
    d.draw_fps(10, 10);
}

// --- NEW: Draw Relics in the top rectangle ---
fn draw_relics(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    for (i, relic) in stats.equipped_relics.iter().enumerate() {
        let x = RELIC_START_X + (i as f32 * RELIC_SPACING);
        let y = RELIC_START_Y;

        let rect = Rectangle::new(x, y, RELIC_SIZE, RELIC_SIZE);

        // Shadow
        d.draw_rectangle_rounded(Rectangle::new(x+4.0, y+4.0, RELIC_SIZE, RELIC_SIZE), 0.2, 4, Color::BLACK.alpha(0.4));
        // Body (Placeholder Color until sprites exist)
        d.draw_rectangle_rounded(rect, 0.2, 4, NEU_BLUE);
        // Border
        d.draw_rectangle_rounded_lines_ex(rect, 0.2, 4, 3.0, NEU_ORANGE);

        // Initial of Relic Name
        let letter = &relic.name[0..1];
        d.draw_text(letter, (x + 30.0) as i32, (y + 25.0) as i32, 40, PARCHMENT);

        // Tooltip on Hover
        let mouse = d.get_mouse_position();
        if rect.check_collision_point_rec(mouse) {
            let tip_w = 250.0;
            let tip_h = 100.0; // Rough height
            let tip_x = x;
            let tip_y = y + RELIC_SIZE + 10.0;

            d.draw_rectangle(tip_x as i32, tip_y as i32, tip_w as i32, tip_h as i32, NEU_BLACK.alpha(0.9));
            d.draw_rectangle_lines(tip_x as i32, tip_y as i32, tip_w as i32, tip_h as i32, NEU_ORANGE);

            d.draw_text(&relic.name, (tip_x + 10.0) as i32, (tip_y + 10.0) as i32, 20, NEU_YELLOW);

            // Simple word wrap for description would go here, for now just printing raw
            d.draw_text(&relic.description, (tip_x + 10.0) as i32, (tip_y + 40.0) as i32, 16, Color::WHITE);
        }
    }
}

// --- NEW: Draw Dice in the right circle ---
fn draw_dice(d: &mut RaylibDrawHandle) {
    let center = DICE_POS;
    let radius = DICE_RADIUS;

    // Shadow
    d.draw_circle((center.x + 4.0) as i32, (center.y + 4.0) as i32, radius, Color::BLACK.alpha(0.4));

    // Dice Body (D20 Placeholder - Hexagon shape ideally, using Circle for now)
    d.draw_circle_v(center, radius, NEU_RED);
    d.draw_circle_lines(center.x as i32, center.y as i32, radius, Color::WHITE);

    // Number (Placeholder)
    d.draw_text("20", (center.x - 15.0) as i32, (center.y - 15.0) as i32, 30, Color::WHITE);

    d.draw_text("SHOP", (center.x - 25.0) as i32, (center.y + 50.0) as i32, 20, PARCHMENT);
}

// --- ANIMATION HELPER ---
fn get_button_offset(d: &RaylibDrawHandle, rect: Rectangle) -> (f32, f32) {
    let mouse_pos = d.get_mouse_position();
    let is_hovered = rect.check_collision_point_rec(mouse_pos);
    let is_down = d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

    if is_hovered {
        if is_down {
            return (4.0, 0.0); // Pressed down
        } else {
            return (-2.0, 6.0); // Hover up
        }
    }
    (0.0, 3.0) // Normal
}

// --- RUNE SELECT SCREEN ---
fn draw_rune_select(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    // LAYOUT CONSTANTS
    let panel_w = 320.0;
    let main_area_w = SCREEN_WIDTH - panel_w;
    let center_x = main_area_w / 2.0;

    // Header
    d.draw_text("RUNE SETUP", (center_x - 120.0) as i32, 30, 50, NEU_ORANGE);
    d.draw_text("Pick one from each row", (center_x - 100.0) as i32, 80, 20, Color::GRAY);

    let rows = [
        (RuneType::Red, "PRIMARY (POWER)", NEU_RED, 130.0),
        (RuneType::Blue, "SECONDARY (UTILITY)", NEU_BLUE, 250.0),
        (RuneType::Green, "TERTIARY (SHOP)", Color::new(76, 175, 80, 255), 370.0),
        (RuneType::Minor, "MINOR STATS", Color::WHITE, 490.0),
    ];

    let mouse_pos = d.get_mouse_position();
    let mut hovered_desc = String::new();
    let mut hovered_name = String::new();

    for (r_type, label, color, start_y) in rows.iter() {
        d.draw_text(label, 50, (*start_y + 30.0) as i32, 20, *color);
        d.draw_line(40, (*start_y + 60.0) as i32, main_area_w as i32 - 40, (*start_y + 60.0) as i32, color.alpha(0.3));

        let row_runes: Vec<_> = stats.available_runes.iter().filter(|r| r.rune_type == *r_type).collect();
        let spacing = 150.0;
        let row_start_x = center_x - ((row_runes.len() as f32 - 1.0) * spacing) / 2.0;

        for (i, rune) in row_runes.iter().enumerate() {
            let x = row_start_x + (i as f32 * spacing);
            let y = *start_y + 30.0;
            let radius = 30.0;

            let is_equipped = stats.equipped_runes.iter().any(|r| r.id == rune.id);
            let final_col = if is_equipped { NEU_YELLOW } else { color.alpha(0.2) };
            let stroke_col = if is_equipped { NEU_ORANGE } else { *color };

            d.draw_circle(x as i32, y as i32, radius, final_col);
            d.draw_circle_lines(x as i32, y as i32, radius, stroke_col);

            let letter = &rune.name[0..1];
            let font_size = 30;
            let text_w = d.measure_text(letter, font_size);
            d.draw_text(letter, (x - text_w as f32 / 2.0) as i32, (y - font_size as f32 / 2.0) as i32, font_size, Color::WHITE);

            if is_equipped {
                d.draw_circle_lines(x as i32, y as i32, radius + 5.0, NEU_ORANGE);
            }

            let dist = ((mouse_pos.x - x).powi(2) + (mouse_pos.y - y).powi(2)).sqrt();
            if dist < radius {
                d.draw_circle_lines(x as i32, y as i32, radius + 3.0, Color::WHITE);
                hovered_name = rune.name.clone();
                hovered_desc = rune.description.clone();
            }
        }
    }

    if !hovered_desc.is_empty() {
        let tip_w = 600.0;
        let tip_x = center_x - tip_w / 2.0;
        let tip_y = 600.0;

        d.draw_rectangle_rounded(Rectangle::new(tip_x, tip_y, tip_w, 80.0), 0.1, 4, NEU_BLACK.alpha(0.95));
        d.draw_rectangle_rounded_lines_ex(Rectangle::new(tip_x, tip_y, tip_w, 80.0), 0.1, 4, 2.0, NEU_ORANGE);
        d.draw_text(&hovered_name, (tip_x + 20.0) as i32, (tip_y + 10.0) as i32, 24, NEU_YELLOW);
        d.draw_text(&hovered_desc, (tip_x + 20.0) as i32, (tip_y + 40.0) as i32, 20, Color::WHITE);
    }

    let panel_x = main_area_w;
    d.draw_rectangle(panel_x as i32, 0, panel_w as i32, SCREEN_HEIGHT as i32, NEU_BLACK);
    d.draw_line_ex(Vector2::new(panel_x, 0.0), Vector2::new(panel_x, SCREEN_HEIGHT), 4.0, NEU_ORANGE);

    d.draw_text("LOADOUT", (panel_x + 20.0) as i32, 40, 40, NEU_ORANGE);
    d.draw_line((panel_x + 20.0) as i32, 85, (SCREEN_WIDTH - 20.0) as i32, 85, Color::GRAY);

    let mut py = 100;
    d.draw_text("ACTIVE RUNES", (panel_x + 20.0) as i32, py, 20, Color::GRAY);
    py += 30;

    for rune in &stats.equipped_runes {
        let col = match rune.rune_type {
            RuneType::Red => NEU_RED,
            RuneType::Blue => NEU_BLUE,
            RuneType::Green => Color::new(76, 175, 80, 255),
            RuneType::Minor => Color::WHITE,
        };
        d.draw_text(&format!("- {}", rune.name), (panel_x + 30.0) as i32, py, 24, col);
        py += 30;
    }

    py = 350;
    d.draw_text("NET EFFECTS", (panel_x + 20.0) as i32, py, 20, Color::GRAY);
    py += 30;
    d.draw_line((panel_x + 20.0) as i32, py - 5, (SCREEN_WIDTH - 20.0) as i32, py - 5, Color::GRAY);

    let mut positives: Vec<String> = Vec::new();
    let mut negatives: Vec<String> = Vec::new();

    for rune in &stats.equipped_runes {
        match rune.name.as_str() {
            "Paladin" => {
                positives.push("+40 Max HP".to_string());
                negatives.push("-20% Mult/Chips".to_string());
            },
            "Reaper" => {
                positives.push("Lifesteal".to_string());
                negatives.push("-5 Max HP".to_string());
            },
            "Judgement" => {
                positives.push("Balance Mult/Chips".to_string());
                negatives.push("Double Enemy HP".to_string());
            },
            "Midas" => {
                positives.push("+25% Gold (Win)".to_string());
                negatives.push("-25% Gold (Loss)".to_string());
            },
            "Greed" => {
                positives.push("+1 Hand Size".to_string());
                positives.push("+1 Discard".to_string());
                negatives.push("-1 Joker Slot".to_string());
            },
            "Investment" => {
                positives.push("Gold Scaling".to_string());
                negatives.push("Less Gold Early".to_string());
            },
            "Merchant" => {
                positives.push("+1 Free Joker".to_string());
                negatives.push("Shop Cost x1.2".to_string());
            },
            "Mentalist" => {
                positives.push("+1 Free Tarot".to_string());
                negatives.push("Tarot Cost x2".to_string());
            },
            "Evolution" => {
                positives.push("+1 Rare Joker".to_string());
                negatives.push("Fast Scaling".to_string());
            },
            "Force" => positives.push("+10 Mult".to_string()),
            "Flow" => positives.push("+10 Chips".to_string()),
            "Wealth" => positives.push("+3 Gold".to_string()),
            _ => {}
        }
    }

    let start_x = (panel_x + 30.0) as i32;

    for p in positives {
        d.draw_text(&p, start_x, py, 20, Color::new(76, 175, 80, 255));
        py += 25;
    }

    for n in negatives {
        d.draw_text(&n, start_x, py, 20, NEU_RED);
        py += 25;
    }

    let start_btn_w = panel_w - 60.0;
    let start_btn_h = 60.0;
    let start_btn_x = panel_x + 30.0;
    let start_btn_y = SCREEN_HEIGHT - 100.0;

    let start_rect = Rectangle::new(start_btn_x, start_btn_y, start_btn_w, start_btn_h);
    let (off, shad) = get_button_offset(d, start_rect);

    d.draw_rectangle_rounded(Rectangle::new(start_rect.x, start_rect.y + shad, start_rect.width, start_rect.height), 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(Rectangle::new(start_rect.x, start_rect.y + off, start_rect.width, start_rect.height), 0.2, 4, NEU_RED);
    d.draw_text("START RUN", (start_rect.x + (start_btn_w - 120.0)/2.0) as i32, (start_rect.y + 20.0 + off) as i32, 20, Color::WHITE);
}

// --- BATTLE RESULT ---
fn draw_battle_result(d: &mut RaylibDrawHandle, _stats: &BaseModifiers) {
    let center_x = SCREEN_WIDTH/2.0;
    let center_y = SCREEN_HEIGHT/2.0;

    d.draw_text("VICTORY!", (center_x - 120.0) as i32, (center_y - 150.0) as i32, 60, NEU_YELLOW);

    d.draw_text("Rewards:", (center_x - 60.0) as i32, (center_y - 50.0) as i32, 30, PARCHMENT);

    d.draw_text("Money: +$5", (center_x - 80.0) as i32, (center_y) as i32, 30, NEU_ORANGE);
    d.draw_text("XP: +50", (center_x - 80.0) as i32, (center_y + 40.0) as i32, 30, NEU_BLUE);

    let btn_rect = Rectangle::new(center_x - 100.0, center_y + 100.0, 200.0, 60.0);
    let (off, shad) = get_button_offset(d, btn_rect);
    d.draw_rectangle_rounded(Rectangle::new(btn_rect.x, btn_rect.y + shad, btn_rect.width, btn_rect.height), 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(Rectangle::new(btn_rect.x, btn_rect.y + off, btn_rect.width, btn_rect.height), 0.2, 4, NEU_ORANGE);
    d.draw_text("CONTINUE", (btn_rect.x + 35.0) as i32, (btn_rect.y + 20.0 + off) as i32, 20, Color::BLACK);
}

// --- SIDEBAR ---
fn draw_sidebar(d: &mut RaylibDrawHandle, stats: &BaseModifiers, anim: &AnimationState, assets: &GameAssets) {
    let sb_w = SIDEBAR_WIDTH;
    let h = SCREEN_HEIGHT;
    // Don't draw opaque background so the main BG shows through
    // d.draw_rectangle(0, 0, sb_w as i32, h as i32, NEU_BLACK);
    // d.draw_line_ex(Vector2::new(sb_w, 0.0), Vector2::new(sb_w, h), 4.0, Color::BLACK);

    // Draw a semi-transparent dark panel so text is readable
    d.draw_rectangle(0, 0, sb_w as i32, h as i32, Color::BLACK.alpha(0.6));
    d.draw_line_ex(Vector2::new(sb_w, 0.0), Vector2::new(sb_w, h), 2.0, NEU_ORANGE.alpha(0.5));

    let pad = 10.0;

    let hp_y = pad + 10.0;
    let label_text = "PLAYER HP";
    let label_w = d.measure_text(label_text, 20);
    d.draw_text(label_text, (pad + 5.0) as i32, hp_y as i32, 20, PARCHMENT);

    let lvl_text = format!("Lvl {}", stats.level);
    let lvl_w = d.measure_text(&lvl_text, 20);
    let lvl_x = sb_w - lvl_w as f32 - 15.0;
    d.draw_text(&lvl_text, lvl_x as i32, hp_y as i32, 20, NEU_YELLOW);

    let bar_start_x = pad + 5.0 + label_w as f32 + 10.0;
    let bar_end_x = lvl_x - 10.0;
    let bar_w = bar_end_x - bar_start_x;

    if bar_w > 0.0 {
        let bar_h = 10.0;
        let bar_y = hp_y + 5.0;
        d.draw_rectangle(bar_start_x as i32, bar_y as i32, bar_w as i32, bar_h as i32, Color::BLACK);
        d.draw_rectangle_lines(bar_start_x as i32, bar_y as i32, bar_w as i32, bar_h as i32, Color::DARKGRAY);
        if stats.xp_target > 0 {
            let xp_pct = (stats.xp as f32 / stats.xp_target as f32).clamp(0.0, 1.0);
            d.draw_rectangle((bar_start_x + 1.0) as i32, (bar_y + 1.0) as i32, ((bar_w - 2.0) * xp_pct) as i32, (bar_h - 2.0) as i32, NEU_BLUE);
        }
    }

    let hp_bar_w = sb_w - pad * 2.0;
    let hp_rect_bg = Rectangle::new(pad, hp_y + 25.0, hp_bar_w, 30.0);
    d.draw_rectangle_rounded(Rectangle::new(pad+2.0, hp_y+27.0, hp_bar_w, 30.0), 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(hp_rect_bg, 0.2, 4, Color::new(30, 20, 20, 255));
    let hp_pct = stats.current_hp as f32 / stats.max_hp as f32;
    let hp_rect_fg = Rectangle::new(pad, hp_y + 25.0, hp_bar_w * hp_pct, 30.0);
    d.draw_rectangle_rounded(hp_rect_fg, 0.2, 4, NEU_RED);
    d.draw_text(&format!("{}/{}", stats.current_hp, stats.max_hp), (pad + 10.0) as i32, (hp_y + 30.0) as i32, 20, PARCHMENT);

    let enemy_y = hp_y + 70.0;
    let blind_rect = Rectangle::new(pad, enemy_y, sb_w - pad*2.0, 120.0);
    d.draw_texture_pro(&assets.tex_panel_blue, Rectangle::new(0.0, 0.0, assets.tex_panel_blue.width as f32, assets.tex_panel_blue.height as f32), Rectangle::new(blind_rect.x+4.0, blind_rect.y+4.0, blind_rect.width, blind_rect.height), Vector2::zero(), 0.0, Color::BLACK.alpha(0.5));
    d.draw_texture_pro(&assets.tex_panel_blue, Rectangle::new(0.0, 0.0, assets.tex_panel_blue.width as f32, assets.tex_panel_blue.height as f32), blind_rect, Vector2::zero(), 0.0, Color::WHITE);
    d.draw_text(&stats.enemy_name, (pad + 15.0) as i32, (enemy_y + 10.0) as i32, 24, NEU_BLACK);

    let ability_text = match stats.active_ability {
        BossAbility::SilenceSuit(_) => "Debuff: Suit",
        BossAbility::HandSizeMinusOne => "Hand Size -1",
        BossAbility::DoubleTarget => "Target 2x",
        BossAbility::PayToDiscard => "Discard Cost $1",
        _ => "",
    };
    if !ability_text.is_empty() {
        d.draw_text(ability_text, (pad + 15.0) as i32, (enemy_y + 40.0) as i32, 20, NEU_RED);
    }

    d.draw_text("Score to Kill:", (pad + 15.0) as i32, (enemy_y + 70.0) as i32, 16, NEU_BLACK);
    d.draw_text(&format!("{}", stats.target_score), (pad + 15.0) as i32, (enemy_y + 90.0) as i32, 24, NEU_BLACK);

    let round_y = enemy_y + 130.0;
    let round_rect = Rectangle::new(pad, round_y, sb_w - pad*2.0, 70.0);
    d.draw_rectangle_rounded(Rectangle::new(pad+2.0, round_y+2.0, sb_w-pad*2.0, 70.0), 0.1, 4, Color::BLACK.alpha(0.5));
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

    let score_y = round_y + 80.0;
    let score_rect = Rectangle::new(pad, score_y, sb_w - pad*2.0, 150.0);
    d.draw_texture_pro(&assets.tex_panel_orange, Rectangle::new(0.0, 0.0, assets.tex_panel_orange.width as f32, assets.tex_panel_orange.height as f32), Rectangle::new(score_rect.x+4.0, score_rect.y+4.0, score_rect.width, score_rect.height), Vector2::zero(), 0.0, Color::BLACK.alpha(0.5));
    d.draw_texture_pro(&assets.tex_panel_orange, Rectangle::new(0.0, 0.0, assets.tex_panel_orange.width as f32, assets.tex_panel_orange.height as f32), score_rect, Vector2::zero(), 0.0, Color::WHITE);
    let hand_name = match stats.hand_rank { Some(r) => format!("{:?}", r), None => "Choose Cards".to_string() };
    d.draw_text(&hand_name, (pad + 20.0) as i32, (score_y + 15.0) as i32, 24, Color::BLACK);

    let time = d.get_time() as f32;
    let mut anim_offset = Vector2::zero();
    let mut text_color = PARCHMENT;

    if stats.chips > 0 || stats.mult > 0 {
        if stats.is_crit_active {
            anim_offset.x = (time * 60.0).sin() * 3.0;
            anim_offset.y = (time * 45.0).cos() * 3.0;
            text_color = NEU_YELLOW;
        } else {
            anim_offset.y = (time * 4.0).sin() * 4.0;
        }
    }

    let calc_box_y = score_y + 50.0;
    let box_w = 80.0;
    let box_h = 60.0;
    let center_offset = 40.0;

    let draw_animated_box = |d: &mut RaylibDrawHandle, base_x: f32, texture: &Texture2D, val: i32, tint: Color| {
        let cx = base_x + anim_offset.x;
        let cy = calc_box_y + anim_offset.y;

        let shadow_dest = Rectangle::new(base_x + 4.0, calc_box_y + 4.0, box_w, box_h);
        d.draw_texture_pro(texture, Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32), shadow_dest, Vector2::zero(), 0.0, Color::BLACK.alpha(0.3));

        let dest = Rectangle::new(cx, cy, box_w, box_h);
        d.draw_texture_pro(texture, Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32), dest, Vector2::zero(), 0.0, tint);

        let txt = format!("{}", val);
        let txt_size = 30;
        let tw = d.measure_text(&txt, txt_size);
        d.draw_text(&txt, (cx + (box_w - tw as f32)/2.0) as i32, (cy + (box_h - txt_size as f32)/2.0) as i32, txt_size, text_color);
    };

    draw_animated_box(d, pad + 10.0 + center_offset, &assets.tex_banner, stats.chips, NEU_BLUE);
    d.draw_text("X", (pad + 100.0 + center_offset) as i32, (calc_box_y + 15.0) as i32, 30, Color::BLACK);
    draw_animated_box(d, pad + 130.0 + center_offset, &assets.tex_banner, stats.mult, NEU_RED);

    let stats_y = 500.0;
    draw_stat_box(d, "Hands", stats.hands_remaining, NEU_BLUE, 45.0, stats_y);
    draw_stat_box(d, "Discards", stats.discards_remaining, NEU_RED, 155.0, stats_y);

    let money_y = stats_y + 80.0;
    let money_rect = Rectangle::new(90.0, money_y, 120.0, 60.0);
    d.draw_rectangle_rounded(Rectangle::new(90.0+3.0, money_y+3.0, 120.0, 60.0), 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(money_rect, 0.2, 4, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(money_rect, 0.2, 4, 3.0, Color::BLACK);
    let money_text = format!("$ {}", stats.money);
    let money_w = d.measure_text(&money_text, 30);
    d.draw_text(&money_text, (90.0 + (120.0 - money_w as f32) / 2.0) as i32, (money_y + 15.0) as i32, 30, Color::BLACK);

    let info_btn_y = money_y + 70.0;
    let info_btn_rect = Rectangle::new(90.0, info_btn_y, 120.0, 40.0);
    let (off, shad) = get_button_offset(d, info_btn_rect);
    let (btn_color, txt_color) = if stats.stat_points > 0 { (NEU_ORANGE, Color::BLACK) } else { (Color::GRAY, PARCHMENT) };
    d.draw_rectangle_rounded(Rectangle::new(info_btn_rect.x, info_btn_rect.y+shad, info_btn_rect.width, info_btn_rect.height), 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(Rectangle::new(info_btn_rect.x, info_btn_rect.y+off, info_btn_rect.width, info_btn_rect.height), 0.2, 4, btn_color);
    let txt = "STATS";
    let txt_w = d.measure_text(txt, 20);
    d.draw_text(txt, (info_btn_rect.x + (120.0 - txt_w as f32)/2.0) as i32, (info_btn_rect.y + 10.0 + off) as i32, 20, txt_color);
}

// --- OVERLAYS ---
fn draw_stats_menu(d: &mut RaylibDrawHandle, stats: &BaseModifiers) {
    let center_x = SCREEN_WIDTH/2.0 - STAT_WIN_W/2.0;
    let center_y = SCREEN_HEIGHT/2.0 - STAT_WIN_H/2.0;
    let rect = Rectangle::new(center_x, center_y, STAT_WIN_W, STAT_WIN_H);

    d.draw_rectangle_rounded(rect, 0.1, 10, NEU_BLACK);
    d.draw_rectangle_rounded_lines_ex(rect, 0.1, 10, 3.0, NEU_ORANGE);

    let title = "RUN INFORMATION";
    let tw = d.measure_text(title, 40);
    d.draw_text(title, (center_x + STAT_WIN_W/2.0 - tw as f32/2.0) as i32, (center_y + 20.0) as i32, 40, NEU_ORANGE);

    let col1_x = center_x + 40.0;
    let start_y = center_y + 80.0;

    d.draw_text("STATS", col1_x as i32, start_y as i32, 24, Color::GRAY);
    d.draw_line(col1_x as i32, (start_y + 30.0) as i32, (col1_x + 250.0) as i32, (start_y + 30.0) as i32, Color::GRAY);

    let pts_text = format!("Stat Points: {}", stats.stat_points);
    d.draw_text(&pts_text, col1_x as i32, (start_y + 40.0) as i32, 20, NEU_YELLOW);

    let x_label = col1_x;
    let x_val = col1_x + 140.0;
    let x_btn = col1_x + 220.0;

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

    draw_stat_row(start_y + 80.0, "Max HP", format!("{}", stats.max_hp));
    draw_stat_row(start_y + 130.0, "Crit Chance", format!("{:.0}%", stats.crit_chance * 100.0));
    draw_stat_row(start_y + 180.0, "Crit Dmg", format!("{:.1}x", stats.crit_mult));

    d.draw_text(&format!("Level: {}", stats.level), col1_x as i32, (start_y + 240.0) as i32, 20, Color::GRAY);
    d.draw_text(&format!("XP: {} / {}", stats.xp, stats.xp_target), col1_x as i32, (start_y + 265.0) as i32, 20, Color::GRAY);

    let col2_x = center_x + 350.0;
    d.draw_text("EQUIPPED RUNES", col2_x as i32, start_y as i32, 24, Color::GRAY);
    d.draw_line(col2_x as i32, (start_y + 30.0) as i32, (col2_x + 250.0) as i32, (start_y + 30.0) as i32, Color::GRAY);

    let mut rune_y = start_y + 50.0;
    if stats.equipped_runes.is_empty() {
        d.draw_text("(None)", col2_x as i32, rune_y as i32, 20, Color::DARKGRAY);
    } else {
        for rune in &stats.equipped_runes {
            let col = match rune.rune_type {
                RuneType::Red => NEU_RED,
                RuneType::Blue => NEU_BLUE,
                RuneType::Green => Color::new(76, 175, 80, 255),
                RuneType::Minor => PARCHMENT,
            };

            d.draw_text(&format!("• {}", rune.name), col2_x as i32, rune_y as i32, 22, col);

            let desc_short = match rune.name.as_str() {
                "Paladin" => "+40 HP / -Stats",
                "Reaper" => "Steal HP / -Max HP",
                "Judgement" => "Bal. Stats / 2x Enemy",
                "Midas" => "+/- 25% Gold",
                "Greed" => "+1 Hand / +1 Disc",
                "Investment" => "Gold Scaling",
                "Merchant" => "+1 Shop Slot / Cost+",
                "Mentalist" => "+1 Tarot / Cost+",
                "Evolution" => "+1 Rare / Scale+",
                "Force" => "+10 Mult",
                "Flow" => "+10 Chips",
                "Wealth" => "+3 Gold",
                _ => "Passive Effect",
            };

            d.draw_text(desc_short, (col2_x + 150.0) as i32, (rune_y + 2.0) as i32, 16, Color::GRAY);
            rune_y += 30.0;
        }
    }

    let back_rect = Rectangle::new(center_x + STAT_WIN_W/2.0 - 100.0, center_y + STAT_WIN_H - 60.0, 200.0, 40.0);
    let (off, shad) = get_button_offset(d, back_rect);
    d.draw_rectangle_rounded(Rectangle::new(back_rect.x, back_rect.y+shad, back_rect.width, back_rect.height), 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(Rectangle::new(back_rect.x, back_rect.y+off, back_rect.width, back_rect.height), 0.2, 4, NEU_BLUE);
    d.draw_text("CLOSE", (back_rect.x + 70.0) as i32, (back_rect.y + 10.0 + off) as i32, 20, PARCHMENT);
}

fn draw_stats_button(d: &mut RaylibDrawHandle, stats: &BaseModifiers, assets: &GameAssets) {
    let rect = Rectangle::new(STATS_BTN_X, STATS_BTN_Y, STATS_BTN_W, STATS_BTN_H);
    let (offset_y, shadow_y) = get_button_offset(d, rect);
    let texture = if stats.stat_points > 0 { &assets.tex_btn_plus_active } else { &assets.tex_btn_plus_disabled };

    let shadow_rect = Rectangle::new(rect.x, rect.y + shadow_y, rect.width, rect.height);
    d.draw_texture_pro(texture, Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32), shadow_rect, Vector2::zero(), 0.0, Color::BLACK.alpha(0.5));

    let anim_rect = Rectangle::new(rect.x, rect.y + offset_y, rect.width, rect.height);
    d.draw_texture_pro(texture, Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32), anim_rect, Vector2::zero(), 0.0, Color::WHITE);
    d.draw_text("+", (anim_rect.x + 12.0) as i32, (anim_rect.y + 8.0) as i32, 30, Color::WHITE);
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
    let (off, shad) = get_button_offset(d, btn_rect);
    d.draw_rectangle_rounded(Rectangle::new(btn_rect.x, btn_rect.y+shad, btn_rect.width, btn_rect.height), 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(Rectangle::new(btn_rect.x, btn_rect.y+off, btn_rect.width, btn_rect.height), 0.2, 4, NEU_BLUE);
    let btn_text = if stats.round_won { "NEXT FIGHT" } else { "RETRY ROUND" };
    let text_w = d.measure_text(btn_text, 20);
    d.draw_text(btn_text, (btn_rect.x + (btn_rect.width - text_w as f32)/2.0) as i32, (btn_rect.y + 20.0 + off) as i32, 20, PARCHMENT);
}

fn draw_stat_box(d: &mut RaylibDrawHandle, label: &str, val: i32, color: Color, x: f32, y: f32) {
    let rect = Rectangle::new(x, y, 100.0, 70.0);
    d.draw_rectangle_rounded(Rectangle::new(rect.x+3.0, rect.y+3.0, rect.width, rect.height), 0.1, 4, Color::BLACK.alpha(0.5));
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

fn draw_game_area(d: &mut RaylibDrawHandle, hand: &[Card], assets: &GameAssets, _stats: &BaseModifiers) {
    let center_x = (SIDEBAR_WIDTH + (SCREEN_WIDTH - SIDEBAR_WIDTH) / 2.0) + CENTER_OFFSET_X;

    // Draw Cards (Behind UI)
    for card in hand { draw_single_card(d, card, assets); }

    // --- SORT BUTTONS (Below Deck) ---
    // Rank Button
    let sort_rank_rect = Rectangle::new(SORT_RANK_POS.x, SORT_RANK_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let (offset, shadow) = get_button_offset(d, sort_rank_rect);
    let r_shadow = Rectangle::new(sort_rank_rect.x, sort_rank_rect.y + shadow, sort_rank_rect.width, sort_rank_rect.height);
    let r_body = Rectangle::new(sort_rank_rect.x, sort_rank_rect.y + offset, sort_rank_rect.width, sort_rank_rect.height);
    d.draw_rectangle_rounded(r_shadow, 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(r_body, 0.2, 4, NEU_ORANGE);
    d.draw_rectangle_rounded_lines_ex(r_body, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text("Rank", (r_body.x + 20.0) as i32, (r_body.y + 8.0) as i32, 16, Color::BLACK);

    // Suit Button
    let sort_suit_rect = Rectangle::new(SORT_SUIT_POS.x, SORT_SUIT_POS.y, SORT_BTN_WIDTH, SORT_BTN_HEIGHT);
    let (offset, shadow) = get_button_offset(d, sort_suit_rect);
    let s_shadow = Rectangle::new(sort_suit_rect.x, sort_suit_rect.y + shadow, sort_suit_rect.width, sort_suit_rect.height);
    let s_body = Rectangle::new(sort_suit_rect.x, sort_suit_rect.y + offset, sort_suit_rect.width, sort_suit_rect.height);
    d.draw_rectangle_rounded(s_shadow, 0.2, 4, Color::BLACK.alpha(0.5));
    d.draw_rectangle_rounded(s_body, 0.2, 4, NEU_BLUE);
    d.draw_rectangle_rounded_lines_ex(s_body, 0.2, 4, 2.0, Color::BLACK);
    d.draw_text("Suit", (s_body.x + 25.0) as i32, (s_body.y + 8.0) as i32, 16, PARCHMENT);

    // --- PLAY / DISCARD BUTTONS ---
    let play_rect = Rectangle::new(center_x - BTN_WIDTH - 10.0, ACTION_BTN_Y, BTN_WIDTH, BTN_HEIGHT);
    let (offset, shadow) = get_button_offset(d, play_rect);
    let p_shadow = Rectangle::new(play_rect.x, play_rect.y + shadow, play_rect.width, play_rect.height);
    d.draw_texture_pro(&assets.tex_btn_play, Rectangle::new(0.0, 0.0, assets.tex_btn_play.width as f32, assets.tex_btn_play.height as f32), p_shadow, Vector2::zero(), 0.0, Color::BLACK.alpha(0.5));
    let p_body = Rectangle::new(play_rect.x, play_rect.y + offset, play_rect.width, play_rect.height);
    d.draw_texture_pro(&assets.tex_btn_play, Rectangle::new(0.0, 0.0, assets.tex_btn_play.width as f32, assets.tex_btn_play.height as f32), p_body, Vector2::zero(), 0.0, Color::WHITE);
    d.draw_text("PLAY", (p_body.x + 35.0) as i32, (p_body.y + 12.0) as i32, 20, Color::WHITE);

    let disc_rect = Rectangle::new(center_x + 10.0, ACTION_BTN_Y, BTN_WIDTH, BTN_HEIGHT);
    let (offset, shadow) = get_button_offset(d, disc_rect);
    let d_shadow = Rectangle::new(disc_rect.x, disc_rect.y + shadow, disc_rect.width, disc_rect.height);
    d.draw_texture_pro(&assets.tex_btn_discard, Rectangle::new(0.0, 0.0, assets.tex_btn_discard.width as f32, assets.tex_btn_discard.height as f32), d_shadow, Vector2::zero(), 0.0, Color::BLACK.alpha(0.5));
    let d_body = Rectangle::new(disc_rect.x, disc_rect.y + offset, disc_rect.width, disc_rect.height);
    d.draw_texture_pro(&assets.tex_btn_discard, Rectangle::new(0.0, 0.0, assets.tex_btn_discard.width as f32, assets.tex_btn_discard.height as f32), d_body, Vector2::zero(), 0.0, Color::WHITE);
    d.draw_text("DISCARD", (d_body.x + 15.0) as i32, (d_body.y + 12.0) as i32, 20, Color::WHITE);

    // --- DRAW DECK ---
    for i in 0..5 {
        let offset = i as f32 * 3.0;
        let rect = Rectangle::new(DECK_X, DECK_Y - offset, CARD_WIDTH * 0.8, CARD_HEIGHT * 0.8);
        d.draw_rectangle_rounded(rect, 0.1, 4, NEU_RED);
        d.draw_rectangle_rounded_lines_ex(rect, 0.1, 4, 2.0, PARCHMENT);
        if i == 4 {
            d.draw_text("DECK", (rect.x + 15.0) as i32, (rect.y + 60.0) as i32, 24, PARCHMENT);
        }
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

    let row_idx = match card.suit {
        0 => 0, // Clubs
        1 => 1, // Diamonds
        2 => 3, // Hearts is Row 3 in your image
        3 => 2, // Spades is Row 2 in your image
        _ => 0,
    };

    let source_rec = Rectangle::new(
        col_idx as f32 * src_w,
        row_idx as f32 * src_h,
        src_w,
        src_h
    );

    let dest_width = CARD_WIDTH * card.scale;
    let dest_height = CARD_HEIGHT * card.scale;

    let dest_rect = Rectangle::new(
        card.current_pos.x,
        card.current_pos.y,
        dest_width,
        dest_height
    );

    let origin = Vector2::new(dest_width / 2.0, dest_height / 2.0);

    let shadow_dist = (card.scale * 10.0).max(5.0);
    let shadow_rect = Rectangle::new(
        dest_rect.x + shadow_dist,
        dest_rect.y + shadow_dist,
        dest_width,
        dest_height
    );

    d.draw_texture_pro(
        &assets.tex_spritesheet,
        source_rec,
        shadow_rect,
        origin,
        card.rotation * 57.29,
        Color::BLACK.alpha(0.5)
    );

    let tint = if card.is_hovered { Color::WHITE } else { Color::new(245, 245, 245, 255) };

    d.draw_texture_pro(
        &assets.tex_spritesheet,
        source_rec,
        dest_rect,
        origin,
        card.rotation * 57.29,
        tint
    );
}