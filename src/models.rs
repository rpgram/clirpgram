use serde::Deserialize;

#[derive(Deserialize)]
pub struct Player {
    pub username: String,
    pub state: u8,
    pub health_points: u8,
    pub effects: Vec<String>,
}

#[derive(Deserialize)]
pub struct Suggestion {
    pub button: u8,
    pub steps_left: u8,
    pub action: String,
}

#[derive(Deserialize)]
pub struct BattleEvent {
    pub side: u8,
    pub action: u8,
    // eid: u128,
}
