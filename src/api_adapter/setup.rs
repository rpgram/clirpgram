use crate::api_adapter::models::PlayerSetupDTO;
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
    pub fn register(&self, username: String) -> Result<PlayerSetupDTO, SetupErrors> {
        let api_url = format!("{}/player", self.backend);
        let q_params = [("username", username)];
        let url = reqwest::Url::parse_with_params(api_url.as_str(), &q_params).unwrap();
        let response = self.client.post(url).send();
        parse_setup_response(response)
    }
}


#[cfg(test)]
mod setup_gateway {
    use crate::api_adapter::setup::SetupClient;

    #[test]
    fn int_registration() {
        let client = reqwest::blocking::Client::new();
        let gateway = SetupClient {backend:"http://localhost:8001", player_id: None, client};
        let chosen_username = "Julian".to_string();
        let new_player = gateway.register(chosen_username.clone());
        assert_eq!(new_player.unwrap().username, chosen_username)
    }
}