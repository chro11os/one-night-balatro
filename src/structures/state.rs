#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Menu,
    Playing,
    Shop,
    StatsMenu,
    Settings,
    GameOver,
    Exit,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationState {
    Idle,
    Playing,     // Cards moving to center
    ScoringSeq,  // NEW: Cards scoring one by one
    Scoring,     // Total numbers counting up
}