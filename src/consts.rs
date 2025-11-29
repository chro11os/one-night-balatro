use raylib::prelude::Color;

// --- ORIGINAL "ONE NIGHT" PALETTE ---
pub const NEU_BG: Color = Color::new(53, 83, 92, 255);
pub const PANEL_BG: Color = Color::new(35, 35, 35, 255);
pub const NEU_BLACK: Color = Color::new(20, 20, 20, 255);

pub const NEU_RED: Color = Color::new(254, 95, 85, 255);
pub const NEU_BLUE: Color = Color::new(0, 157, 255, 255);
pub const NEU_YELLOW: Color = Color::new(255, 204, 0, 255);
pub const NEU_ORANGE: Color = Color::new(255, 140, 0, 255); // Added for Play Button

pub const SUIT_RED: Color = NEU_RED;
pub const SUIT_BLACK: Color = Color::new(40, 40, 40, 255);
pub const CARD_BG: Color = Color::RAYWHITE;

// --- LAYOUT ---
pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

pub const CARD_WIDTH: f32 = 80.0;
pub const CARD_HEIGHT: f32 = 112.0;

pub const HAND_Y_POS: f32 = 580.0;
pub const SELECTED_Y_POS: f32 = 550.0;
pub const ANIM_SPEED: f32 = 18.0;

pub const DECK_X: f32 = 1150.0;
pub const DECK_Y: f32 = 600.0;

pub const SHADOW_OFFSET: f32 = 6.0;

// --- BUTTONS ---
pub const BTN_WIDTH: f32 = 160.0;
pub const BTN_HEIGHT: f32 = 60.0;
// Positioned above the hand
pub const PLAY_BTN_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: 450.0, y: 450.0 };
pub const DISC_BTN_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: 630.0, y: 450.0 };