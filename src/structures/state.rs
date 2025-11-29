#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Menu,
    Playing,
    Settings,
    GameOver,
    Exit,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationState {
    Idle,
    Playing, // Cards moving to center
    Scoring, // New: Numbers counting up
}