use raylib::prelude::Color;

// --- DUNGEON PALETTE ---
pub const NEU_BG: Color = Color::new(38, 35, 34, 255);
pub const NEU_BLACK: Color = Color::new(20, 18, 18, 255);
pub const NEU_RED: Color = Color::new(168, 45, 45, 255);
pub const NEU_BLUE: Color = Color::new(75, 107, 140, 255);
pub const NEU_ORANGE: Color = Color::new(218, 165, 32, 255);
pub const NEU_YELLOW: Color = Color::new(255, 215, 0, 255);
pub const PARCHMENT: Color = Color::new(235, 225, 205, 255);
pub const NEU_GREEN: Color = Color::new(76, 175, 80, 255);

// --- LAYOUT (1600x900) ---
pub const SCREEN_WIDTH: f32 = 1600.0;
pub const SCREEN_HEIGHT: f32 = 900.0;

// Card Dimensions
pub const CARD_WIDTH: f32 = 140.0;
pub const CARD_HEIGHT: f32 = 190.0;

// SCALES
pub const HAND_SCALE: f32 = 0.85;
pub const SELECTED_SCALE: f32 = 1.0;
pub const PLAYED_SCALE: f32 = 0.65;
pub const JUNK_SCALE: f32 = 0.70;

// --- POSITIONS ---

// 1. ENEMY PANEL (Top Center)
pub const ENEMY_CENTER_X: f32 = SCREEN_WIDTH / 2.0;
pub const ENEMY_Y: f32 = 50.0;

// 2. PLAYER PANEL (Bottom Left)
pub const P_PANEL_X: f32 = 40.0;
pub const P_PANEL_Y: f32 = SCREEN_HEIGHT - 260.0;
pub const P_PANEL_W: f32 = 300.0;
pub const P_PANEL_H: f32 = 220.0;

// 3. ACTION PANEL (Bottom Right)
pub const A_PANEL_X: f32 = SCREEN_WIDTH - 340.0;
pub const A_PANEL_Y: f32 = SCREEN_HEIGHT - 260.0;
pub const A_PANEL_W: f32 = 300.0;
pub const A_PANEL_H: f32 = 220.0;

// Buttons inside Action Panel
pub const BTN_WIDTH: f32 = 130.0;
pub const BTN_HEIGHT: f32 = 55.0;
pub const PLAY_BTN_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: A_PANEL_X + 15.0, y: A_PANEL_Y + 15.0 };
pub const DISC_BTN_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: A_PANEL_X + 155.0, y: A_PANEL_Y + 15.0 };

// Scoring Box inside Action Panel
pub const SCORE_BOX_Y: f32 = A_PANEL_Y + 85.0;

// 4. CARDS (Bottom Center)
pub const HAND_Y_POS: f32 = SCREEN_HEIGHT - 150.0;
pub const PLAYED_Y_POS: f32 = SCREEN_HEIGHT / 2.0;

// 5. SORT BUTTONS (Above Action Panel)
pub const SORT_RANK_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: A_PANEL_X, y: A_PANEL_Y - 50.0 };
pub const SORT_SUIT_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: A_PANEL_X + 160.0, y: A_PANEL_Y - 50.0 };
pub const SORT_BTN_WIDTH: f32 = 140.0;
pub const SORT_BTN_HEIGHT: f32 = 40.0;

// 6. DECK POSITION
pub const DECK_X: f32 = A_PANEL_X + 20.0;
pub const DECK_Y: f32 = A_PANEL_Y - 150.0;

// 7. RUNE LAYOUT (SHARED)
// Using shared constants prevents click misalignment!
pub const RUNE_CONTENT_OFFSET: f32 = -250.0;
pub const RUNE_START_Y: f32 = 200.0;
pub const RUNE_SPACING_X: f32 = 100.0;
pub const RUNE_SPACING_Y: f32 = 140.0;
pub const RUNE_RADIUS: f32 = 32.0;

// --- MISC ---
pub const RELIC_START_X: f32 = 50.0;
pub const RELIC_START_Y: f32 = 20.0;
pub const RELIC_SPACING: f32 = 80.0;
pub const RELIC_SIZE: f32 = 60.0;

pub const STATS_BTN_X: f32 = P_PANEL_X + P_PANEL_W - 50.0;
pub const STATS_BTN_Y: f32 = P_PANEL_Y - 40.0;
pub const STATS_BTN_W: f32 = 50.0;
pub const STATS_BTN_H: f32 = 30.0;

pub const STAT_WIN_W: f32 = 700.0;
pub const STAT_WIN_H: f32 = 450.0;

pub const MENU_BTN_START_Y: f32 = 300.0;
pub const MENU_BTN_GAP: f32 = 10.0;

pub const DEV_BOX_X: f32 = 20.0;
pub const DEV_BOX_Y: f32 = 20.0;
pub const DEV_BTN_W: f32 = 180.0;
pub const DEV_BTN_H: f32 = 40.0;
pub const DEV_GAP: f32 = 10.0;