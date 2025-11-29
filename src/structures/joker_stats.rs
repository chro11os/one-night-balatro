#[derive(Debug)]
// UPDATE: Renamed struct to PascalCase to follow Rust conventions
pub struct JokerBaseModifiers
{
    pub joker_name: String,
    pub mult: i32,
    pub chips: i32,
    pub gold: i32,
    pub hands: i32,
    pub discards: i32,
    pub joker_slot: i32,
}