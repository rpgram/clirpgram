use crate::domain::types::{BattleId, PlayerId};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BattleStarted {
    pub start_time: Option<BattleId>,
    battle_id: u64,
}

#[derive(Deserialize)]
pub struct WaitingBattle {
    pub battle_id: BattleId,
    pub player_id: PlayerId,
    // todo here will be also hero
}

#[derive(Deserialize)]
pub struct PlayerResult {
    pub player_id: u16,
    is_hero: bool,
    pub win: bool,
}

#[derive(Deserialize)]
pub struct BattleResult {
    pub hero_result: PlayerResult,
    opponent_result: PlayerResult,
}

#[derive(Deserialize)]
pub struct PlayerDTO {
    pub username: String,
    pub player_id: PlayerId,
}

#[derive(Deserialize)]
pub struct JSONPacket {
    pub track_id: u32,
    pub ts: u64,
    pub dur: u64,
    pub trim_start: u32,
    pub trim_end: u32,
    pub data: Vec<u8>,
}

#[derive(Deserialize)]
struct GoodDTO {
    price: Vec<String>,
    quantity: u8,
    name: String,
}

pub type HeroClass = u8;

#[derive(Deserialize)]
pub struct PlayerSetupDTO {
    pub balance: String,
    pub inventory: Vec<GoodDTO>,
    pub heroes: Vec<HeroClass>,
    pub username: String,
    pub player_id: PlayerId,
}
