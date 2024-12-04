use ratatui::prelude::StatefulWidget;
use reqwest::blocking::{Client};
pub struct APICaller {
    world_back: String,
    battle_back: String,
    client: Client,
}


impl APICaller {
    pub fn new(world_url: String, battle_url: String) -> Self {
        Self {
            world_back: world_url,
            battle_back: battle_url,
            client: Client::new()
        }
    }
}