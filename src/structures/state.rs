#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Menu,
    RuneSelect, // NEW: Before playing
    Playing,
    BattleResult,
    Shop,
    StatsMenu,
    Settings,
    GameOver,
    Exit,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationState {
    Idle,
    Playing,
    ScoringSeq,
    Scoring,
}