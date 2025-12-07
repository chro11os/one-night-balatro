use raylib::prelude::*;
use crate::structures::stats::BaseModifiers;
use crate::structures::card::Card;
use crate::structures::assets::GameAssets;
use crate::structures::state::{GameState, AnimationState};
use crate::consts::*;

// Module declarations
mod game_screen;
mod shop_screen;
mod menu_screen;
mod rune_select_screen;
mod stats_menu_screen;
mod battle_result_screen;
pub mod ui_elements;

// Re-exporting functions to be used by main
use game_screen::*;
use shop_screen::*;
use menu_screen::*;
use rune_select_screen::*;
use stats_menu_screen::*;
use battle_result_screen::*;



pub fn draw_scene(d: &mut RaylibDrawHandle, stats: &BaseModifiers, hand: &[Card], state: &GameState, assets: &GameAssets, anim: &AnimationState) {
    let camera = Camera2D {
        target: Vector2::new(0.0, 0.0),
        offset: stats.shake_offset,
        rotation: stats.shake_rotation,
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
                draw_relics(&mut d_cam, stats, assets);
                draw_game_area(&mut d_cam, hand, assets, stats);

                // FIX: Scoring Popups now use Custom Font
                for ft in &stats.floating_texts {
                    let alpha = (ft.life / ft.max_life).clamp(0.0, 1.0);
                    let color = ft.color.alpha(alpha);
                    // Shadow
                    d_cam.draw_text_ex(&assets.font_main, &ft.text, Vector2::new(ft.pos.x + 2.0, ft.pos.y + 2.0), ft.size as f32, 1.0, Color::BLACK.alpha(alpha));
                    // Main Text
                    d_cam.draw_text_ex(&assets.font_main, &ft.text, Vector2::new(ft.pos.x, ft.pos.y), ft.size as f32, 1.0, color);
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
            draw_rune_select(d, stats, assets);
        },
        GameState::Shop => {
            // Draw the playing state as a background
            let src = Rectangle::new(0.0, 0.0, assets.tex_background.width as f32, assets.tex_background.height as f32);
            let dest = Rectangle::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT);
            d.draw_texture_pro(&assets.tex_background, src, dest, Vector2::zero(), 0.0, Color::WHITE);
            {
                let mut d_cam = d.begin_mode2D(camera);
                draw_enemy_monitor(&mut d_cam, stats, assets);
                draw_player_panel(&mut d_cam, stats, assets);
                draw_action_panel(&mut d_cam, stats, anim, assets, hand);
                draw_sort_buttons(&mut d_cam, assets);
                draw_relics(&mut d_cam, stats, assets);
                draw_game_area(&mut d_cam, hand, assets, stats);
            }
            // Draw a semi-transparent overlay
            d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, NEU_BLACK.alpha(0.85));
            // Draw the shop UI
            draw_shop(d, stats, assets);
        },
        GameState::StatsMenu => {
            {
                let mut d_cam = d.begin_mode2D(camera);
                draw_player_panel(&mut d_cam, stats, assets);
            }
            d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, NEU_BLACK.alpha(0.85));
            draw_stats_menu(d, stats, assets);
        },
        GameState::BattleResult => {
            // Draw the playing state as a background
            let src = Rectangle::new(0.0, 0.0, assets.tex_background.width as f32, assets.tex_background.height as f32);
            let dest = Rectangle::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT);
            d.draw_texture_pro(&assets.tex_background, src, dest, Vector2::zero(), 0.0, Color::WHITE);
            {
                let mut d_cam = d.begin_mode2D(camera);
                draw_enemy_monitor(&mut d_cam, stats, assets);
                draw_player_panel(&mut d_cam, stats, assets);
                draw_action_panel(&mut d_cam, stats, anim, assets, hand);
                draw_sort_buttons(&mut d_cam, assets);
                draw_relics(&mut d_cam, stats, assets);
                draw_game_area(&mut d_cam, hand, assets, stats);
            }
            // Draw a semi-transparent overlay
            d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, NEU_BLACK.alpha(0.85));
            // Draw the battle result UI
            draw_battle_result(d, stats, assets);
        },
        GameState::Menu => {
            d.clear_background(NEU_BG);
            draw_menu(d, assets); // Assuming you want main menu to use it too if not already
        },
        _ => d.clear_background(NEU_BG),
    }
    d.draw_fps(10, 10);
}
