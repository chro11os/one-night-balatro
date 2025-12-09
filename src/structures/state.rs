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
    Dealing,
    Playing, // Old one, might be unused
    PlayingAnimation, // NEW: For the fly-to-center sequence
    ScoringSeq,
    Scoring,
    Discarding,
}