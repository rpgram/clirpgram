use crate::entities::types::{BattleId, PlayerId};


pub enum KeyToUI {
    Up,
    Down,
    Choose,
    Shutdown
}

#[derive(Clone)]
pub enum Action {
    StartInput,
    StartBattle(Option<PlayerId>),
    ConnectToBattle,
    ChooseToConnect(BattleId),
}

#[derive(Clone)]
pub struct MenuChoice {
    pub action: Action,
    pub title: String,
}

pub enum MenuTag {
    MainMenu,
    ConnectMenu,
    StartMenu,
}