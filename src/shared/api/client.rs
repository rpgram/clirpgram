use reqwest::blocking::{Client};
use crate::entities::types::{BattleId, PlayerId};
use crate::shared::api::models::{BattleResult, BattleStarted, JSONPacket, PlayerDTO, WaitingBattle};
use crate::shared::api::utils::parse_response;
use crate::ui::battlefield::BattleField;

pub struct APIClient<'a> {
    pub backend: &'a str,
    pub player_id: PlayerId,
    pub client: Client,
}

impl<'a> APIClient<'_> {
    pub fn leave_battle(&self) -> bool {
        let url = format!("{}/leave", self.backend);
        let response = self.client.delete(url).send();
        match response {
            Ok(data) => data.json().unwrap_or(false),
            Err(_) => false,
        }
    }
    pub fn get_battlefield(&self) -> Option<BattleField> {
        let mut url = self.backend.to_string();
        url.push_str("/client");
        let query = [("player_id", self.player_id.to_string())];
        let reqwest_url = reqwest::Url::parse_with_params(url.as_str(), &query).unwrap();
        let response = self.client.get(reqwest_url).send();
        parse_response::<BattleField>(response)
    }

    pub fn start_battle(&self, opponent_id: Option<PlayerId>) -> Option<BattleStarted> {
        let url = self.backend.to_string();
        let mut params = vec![("player_id", self.player_id.to_string())];
        match opponent_id {
            None => {}
            Some(id) => params.push(("opponent_id", id.to_string())),
        };
        let reqwest_url = reqwest::Url::parse_with_params(url.as_str(), &params).unwrap();
        let response = self.client.post(reqwest_url).send();
        parse_response(response)
    }

    pub fn get_waiting_battles(&self) -> Vec<WaitingBattle> {
        let response = self.client.get(self.backend).send().unwrap();
        response.json().unwrap()
    }

    pub fn connect_to_battle(&self, battle_id: BattleId) -> u64 {
        let url = format!("{}/connect", self.backend);
        let params = [
            ("battle_id", battle_id.to_string()),
            ("player_id", self.player_id.to_string()),
        ];
        let reqwest_url = reqwest::Url::parse_with_params(url.as_str(), &params).unwrap();
        let response = self.client.post(reqwest_url).send();
        parse_response(response).unwrap_or(0)
    }

    pub fn press_key(&self, key: char) -> bool {
        let mut url = self.backend.to_string();
        url.push_str(format!("/{key}").as_str());
        let query = [("player_id", self.player_id.to_string())];
        let reqwest_url = reqwest::Url::parse_with_params(url.as_str(), &query).unwrap();
        let response = self.client.post(reqwest_url).send();
        match response {
            Ok(_) => true,
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }

    pub fn get_players(&self) -> Vec<PlayerDTO> {
        let url = format!("{}/players", self.backend);
        self.client.get(url).send().unwrap().json().unwrap()
    }

    pub fn get_result(&self) -> Option<BattleResult> {
        let mut url = self.backend.to_string();
        url.push_str("/result");
        let params = [("player_id", self.player_id.to_string())];
        let reqwest_url = reqwest::Url::parse_with_params(url.as_str(), &params).unwrap();
        let response = self.client.get(reqwest_url).send();
        parse_response(response)
    }

    pub fn get_audio_buf(&self) -> Vec<JSONPacket> {
        let url = "http://localhost:3000/main/";
        let response = self.client.get(url).send().unwrap();
        response.json().unwrap_or_default()
    }

    // pub fn get_sample()
}

#[cfg(test)]
mod client {
    // use crate::api_adapter::client::get_battlefield;
    use reqwest::blocking::Client;

    #[test]
    fn calls() {
        // let client = Client::new();
        // let _ = get_battlefield(&client, "http://localhost:8000/fake/battle");
    }
}
