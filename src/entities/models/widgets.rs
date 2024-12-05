use crate::entities::api::models::{BattleEvent, Player, Suggestion};
use crate::entities::types::BattleId;

pub struct BattleField {
    pub battle_id: BattleId,
    player: Player,
    opponent: Player,
    next_move: Vec<Suggestion>,
    complete_actions: Vec<BattleEvent>,
    moves: Vec<u8>,
}