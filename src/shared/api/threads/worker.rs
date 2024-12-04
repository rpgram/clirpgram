use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use crate::entities::models::app::AppState;
use crate::entities::types::PlayerId;
use crate::shared::api::threads::client::APICaller;
use crate::shared::config::app::Config;

pub enum APIQuery {
    Register(String, String, String),
    Login(String, String),
    GetHero(u8),
    StartBattle(PlayerId),
    BattleClick(char),
}

pub fn work(app_state: Arc<Mutex<AppState>>, rc: Receiver<APIQuery>, config: Config) {
    let gateway = APICaller::new(config.world_url, config.battlefield_url);
    loop {
        match rc.recv().unwrap() {
            APIQuery::Register(login, pass, username) => {}
            APIQuery::Login(login, pass) => {}
            APIQuery::GetHero(hero_id) => {}
            APIQuery::StartBattle(opponent_id) => {}
            APIQuery::BattleClick(key) => {}
        }
    }
}