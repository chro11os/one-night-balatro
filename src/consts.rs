use raylib::prelude::Color;

// --- DUNGEON PALETTE ---
pub const NEU_BG: Color = Color::new(38, 35, 34, 255);
pub const NEU_BLACK: Color = Color::new(20, 18, 18, 255);
pub const NEU_RED: Color = Color::new(168, 45, 45, 255);
pub const NEU_BLUE: Color = Color::new(75, 107, 140, 255);
pub const NEU_ORANGE: Color = Color::new(218, 165, 32, 255);
pub const NEU_YELLOW: Color = Color::new(255, 215, 0, 255);
pub const PARCHMENT: Color = Color::new(235, 225, 205, 255);

// --- CARD TINTS ---
pub const TINT_CLUBS: Color = Color::new(180, 210, 255, 255);
pub const TINT_DIAMONDS: Color = Color::new(255, 235, 160, 255);
pub const TINT_HEARTS: Color = Color::new(255, 200, 200, 255);
pub const TINT_SPADES: Color = Color::new(220, 220, 230, 255);

// --- LAYOUT ---
pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const SIDEBAR_WIDTH: f32 = 300.0;

pub const CARD_WIDTH: f32 = 140.0;
pub const CARD_HEIGHT: f32 = 190.0;

// SCALES
pub const HAND_SCALE: f32 = 0.85;
pub const SELECTED_SCALE: f32 = 1.0;
pub const PLAYED_SCALE: f32 = 0.65;
pub const JUNK_SCALE: f32 = 0.70;

// POSITIONS
pub const DECK_X: f32 = 1145.0;
pub const DECK_Y: f32 = 460.0;

pub const RELIC_START_X: f32 = 320.0;
pub const RELIC_START_Y: f32 = 30.0;
pub const RELIC_SPACING: f32 = 90.0;
pub const RELIC_SIZE: f32 = 70.0;

pub const DICE_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: 1210.0, y: 320.0 };
pub const DICE_RADIUS: f32 = 45.0;

pub const HAND_Y_POS: f32 = 530.0;
pub const PLAYED_Y_POS: f32 = 250.0;

// --- BUTTONS ---
pub const BTN_WIDTH: f32 = 120.0;
pub const BTN_HEIGHT: f32 = 45.0;
pub const SORT_BTN_WIDTH: f32 = 80.0;
pub const SORT_BTN_HEIGHT: f32 = 30.0;

pub const ACTION_BTN_Y: f32 = 660.0;

pub const CENTER_OFFSET_X: f32 = -60.0;

// UPDATE: New positions for Sort Buttons (Below Deck)
// Deck is ~112px wide. Buttons are 80px.
// Centered roughly under deck: 1145 + (112-80)/2 = 1161.
// Deck Bottom is approx 612px (460 + 152).
pub const SORT_RANK_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: 1161.0, y: 630.0 };
pub const SORT_SUIT_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: 1161.0, y: 670.0 };

pub const STATS_BTN_X: f32 = 1100.0;
pub const STATS_BTN_Y: f32 = 600.0;
pub const STATS_BTN_W: f32 = 140.0;
pub const STATS_BTN_H: f32 = 60.0;

pub const STAT_WIN_W: f32 = 700.0;
pub const STAT_WIN_H: f32 = 450.0;

pub const MENU_BTN_START_Y: f32 = 300.0;
pub const MENU_BTN_GAP: f32 = 10.0;

// --- DEV TOOLBOX ---
pub const DEV_BOX_X: f32 = 1050.0;
pub const DEV_BOX_Y: f32 = 50.0;
pub const DEV_BTN_W: f32 = 180.0;
pub const DEV_BTN_H: f32 = 40.0;
pub const DEV_GAP: f32 = 10.0;