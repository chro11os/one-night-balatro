use raylib::prelude::Color;

// --- PALETTE ---
pub const NEU_BG: Color = Color::new(53, 53, 53, 255);
pub const NEU_BLACK: Color = Color::new(30, 30, 30, 255);
pub const NEU_RED: Color = Color::new(254, 95, 85, 255);
pub const NEU_BLUE: Color = Color::new(0, 157, 255, 255);
pub const NEU_ORANGE: Color = Color::new(255, 165, 0, 255);
pub const NEU_YELLOW: Color = Color::new(255, 204, 0, 255); // New for Rank sort

// --- LAYOUT ---
pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const SIDEBAR_WIDTH: f32 = 300.0;

// Card Size
pub const CARD_WIDTH: f32 = 140.0;
pub const CARD_HEIGHT: f32 = 190.0;

// SCALES
pub const HAND_SCALE: f32 = 0.85;
pub const SELECTED_SCALE: f32 = 1.0;
pub const PLAYED_SCALE: f32 = 0.95;

// POSITIONS
pub const DECK_X: f32 = 1150.0;
pub const DECK_Y: f32 = 500.0;

pub const HAND_Y_POS: f32 = 500.0;
pub const PLAYED_Y_POS: f32 = 250.0;

// --- BUTTONS ---
pub const BTN_WIDTH: f32 = 120.0;
pub const BTN_HEIGHT: f32 = 45.0;

// NEW: Sort Buttons (Smaller)
pub const SORT_BTN_WIDTH: f32 = 80.0;
pub const SORT_BTN_HEIGHT: f32 = 30.0;

// Positions handled in logic/draw
pub const PLAY_BTN_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: 0.0, y: 0.0 };
pub const DISC_BTN_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: 0.0, y: 0.0 };

pub const MENU_BTN_START_Y: f32 = 300.0;
pub const MENU_BTN_GAP: f32 = 10.0;