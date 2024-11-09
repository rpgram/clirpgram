use crate::api_adapter::models::{HeroClass, PlayerSetupDTO};
use crate::api_adapter::utils::{parse_setup_response};
use crate::domain::types::PlayerId;
use reqwest::blocking::Client;

pub struct SetupClient<'a> {
    backend: &'a str,
    player_id: Option<PlayerId>,
    client: Client,
}

#[derive(Debug)]
pub enum SetupErrors {
    UnknownError(u16, String),
    NotUnique,
    ServiceError,
}

impl SetupClient<'_> {
    pub fn register(&mut self, username: String) -> Result<PlayerSetupDTO, SetupErrors> {
        let api_url = format!("{}/player", self.backend);
        let q_params = [("username", username)];
        let url = reqwest::Url::parse_with_params(api_url.as_str(), &q_params).unwrap();
        let response = self.client.post(url).send();
        let new_player = parse_setup_response::<PlayerSetupDTO>(response)?;
        self.player_id = Some(new_player.player_id);
        Ok(new_player)
    }

    pub fn get_hero(&self, hero_class: HeroClass) {
        let api_url = format!("{}/hero", self.backend);
        let q_param = [("player_id", self.player_id.expect("Not authenticated.").to_string()),
            ("hero_class",hero_class.to_string())];
        let reqwest_url = reqwest::Url::parse_with_params(api_url.as_str(), &q_param);
        let _response = self.client.post(reqwest_url.unwrap()).send();
    }

    pub fn get_player(&self, player: Option<PlayerId>) -> Result<PlayerSetupDTO, SetupErrors> {
        let player_id;
        match player {
            None => {player_id = self.player_id.expect("Not authenticated");}
            Some(id) => {player_id = id;}
        }
        let api_url = format!("{}/player/by", self.backend);
        let q_params = [("player_id", player_id.to_string())];
        let reqwest_url = reqwest::Url::parse_with_params(api_url.as_str(), q_params);
        let response = self.client.get(reqwest_url.unwrap()).send();
        parse_setup_response::<PlayerSetupDTO>(response)
    }
}


#[cfg(test)]
mod setup_gateway {
    use crate::api_adapter::setup::SetupClient;

    #[test]
    fn integration_test() {
        let client = reqwest::blocking::Client::new();
        let mut gateway = SetupClient {backend: "http://localhost:8001", player_id: None, client};
        let chosen_username = "Julian".to_string();
        let new_player = gateway.register(chosen_username.clone());
        assert_eq!(new_player.unwrap().username, chosen_username);
        gateway.get_hero(1);
        let updated_player = gateway.get_player(None).unwrap();
        assert!(updated_player.heroes.contains(&1));
    }
}