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

pub const CARD_WIDTH: f32 = 140.0;
pub const CARD_HEIGHT: f32 = 190.0;

pub const HAND_SCALE: f32 = 0.85;
pub const SELECTED_SCALE: f32 = 1.0;
pub const PLAYED_SCALE: f32 = 0.65;
pub const JUNK_SCALE: f32 = 0.70;

// --- POSITIONS ---
pub const ENEMY_CENTER_X: f32 = SCREEN_WIDTH / 2.0;
pub const ENEMY_Y: f32 = 50.0;

// NEW: Vertical position for cards during the scoring animation
pub const PLAY_AREA_Y: f32 = 380.0;

// Player Panel
pub const P_PANEL_X: f32 = 40.0;
pub const P_PANEL_Y: f32 = SCREEN_HEIGHT - 260.0;
pub const P_PANEL_W: f32 = 300.0;
pub const P_PANEL_H: f32 = 220.0;

// Action Panel
pub const A_PANEL_X: f32 = SCREEN_WIDTH - 340.0;
pub const A_PANEL_Y: f32 = SCREEN_HEIGHT - 260.0;
pub const A_PANEL_W: f32 = 300.0;
pub const A_PANEL_H: f32 = 220.0;

pub const BTN_WIDTH: f32 = 130.0;
pub const BTN_HEIGHT: f32 = 55.0;
pub const PLAY_BTN_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: A_PANEL_X + 15.0, y: A_PANEL_Y + 15.0 };
pub const DISC_BTN_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: A_PANEL_X + 155.0, y: A_PANEL_Y + 15.0 };

pub const SCORE_BOX_Y: f32 = A_PANEL_Y + 110.0;

pub const HAND_Y_POS: f32 = SCREEN_HEIGHT - 150.0;
pub const PLAYED_Y_POS: f32 = SCREEN_HEIGHT / 2.0;

pub const SORT_RANK_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: A_PANEL_X, y: A_PANEL_Y - 50.0 };
pub const SORT_SUIT_POS: raylib::math::Vector2 = raylib::math::Vector2 { x: A_PANEL_X + 160.0, y: A_PANEL_Y - 50.0 };
pub const SORT_BTN_WIDTH: f32 = 140.0;
pub const SORT_BTN_HEIGHT: f32 = 40.0;

pub const DECK_X: f32 = A_PANEL_X + 20.0;
pub const DECK_Y: f32 = A_PANEL_Y - 150.0;

pub const RUNE_CONTENT_OFFSET: f32 = -250.0;
pub const RUNE_START_Y: f32 = 200.0;
pub const RUNE_SPACING_X: f32 = 100.0;
pub const RUNE_SPACING_Y: f32 = 140.0;
pub const RUNE_RADIUS: f32 = 32.0;

pub const RELIC_START_X: f32 = 50.0;
pub const RELIC_START_Y: f32 = 20.0;
pub const RELIC_SPACING: f32 = 80.0;
pub const RELIC_SIZE: f32 = 60.0;

pub const STATS_BTN_RECT: raylib::math::Rectangle = raylib::math::Rectangle {
    x: P_PANEL_X + 10.0,
    y: P_PANEL_Y + 160.0,
    width: P_PANEL_W - 20.0,
    height: 50.0
};

// Unused but kept to prevent errors if referenced elsewhere
pub const STAT_WIN_W: f32 = 700.0;
pub const STAT_WIN_H: f32 = 450.0;
pub const MENU_BTN_START_Y: f32 = 300.0;
pub const MENU_BTN_GAP: f32 = 10.0;
pub const DEV_BOX_X: f32 = 20.0;
pub const DEV_BOX_Y: f32 = 20.0;
pub const DEV_BTN_W: f32 = 180.0;
pub const DEV_BTN_H: f32 = 40.0;
pub const DEV_GAP: f32 = 10.0;

// --- SHOP LAYOUT ---
pub const SHOP_ITEM_W: f32 = 300.0;
pub const SHOP_ITEM_H: f32 = 400.0;
pub const SHOP_ITEM_GAP: f32 = 50.0;
pub const SHOP_START_X: f32 = (SCREEN_WIDTH - (3.0 * SHOP_ITEM_W + 2.0 * SHOP_ITEM_GAP)) / 2.0;
pub const SHOP_START_Y: f32 = 200.0;
pub const SHOP_LEAVE_BTN_RECT: raylib::math::Rectangle = raylib::math::Rectangle {
    x: SCREEN_WIDTH / 2.0 - 100.0,
    y: SCREEN_HEIGHT - 100.0,
    width: 200.0,
    height: 50.0
};