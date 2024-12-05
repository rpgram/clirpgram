use std::time::{Duration, UNIX_EPOCH};
use std::{thread, time};
use crate::application::start_battle::running_battle;
use crate::entities::models::menu::{Action, MenuTag};
use crate::entities::types::PlayerId;
use crate::pages::interface::IMenu;
use crate::shared::api::client::APIClient;
use crate::ui::pages::menu::get_action;
use crate::ui::screen::Screen;

#[derive(Clone)]
pub struct StartBattleMenu {}

impl IMenu for StartBattleMenu {
    fn choose(&self, screen: &mut Screen) {
        match get_action(screen) {
            Action::StartInput => {}
            Action::StartBattle(player_id) => {
                StartBattleMenu::start_battle(&screen.api_client, player_id);
                running_battle(screen, Duration::from_millis(300));
            }
            Action::ConnectToBattle => {}
            Action::ChooseToConnect(_) => {}
        }
        panic!()
    }

    fn self_type(&self) -> MenuTag {
        MenuTag::StartMenu
    }
}

impl StartBattleMenu {
    fn start_battle(api_client: &APIClient, opponent_id: Option<PlayerId>) {
        let battle_started = api_client.start_battle(opponent_id);
        match battle_started {
            None => {}
            Some(bs) => match bs.start_time {
                None => {}
                Some(start) => {
                    let ts = time::SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let delta_sec: i128 = start as i128 - ts as i128;
                    if delta_sec > 0 {
                        thread::sleep(Duration::from_secs(delta_sec as u64))
                    }
                }
            },
        }
    }
}
