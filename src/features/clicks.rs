use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use crate::entities::models::menu::KeyToUI;
use crate::pages::interface::IMenu;
use crate::shared::api::threads::worker::APIQuery;
use crate::shared::config::clicks::KeyConfig;
use crate::shared::config::consts::CLICK_REACTION_MS;
use crate::widgets::controller::UIController;

pub fn take_keys(sender: Sender<APIQuery>, ui_sender: Sender<KeyToUI>, key_config: KeyConfig) {
    loop {
        if let Ok(ok) = event::poll(Duration::from_millis(CLICK_REACTION_MS)) {
            if ok == false {
                continue
            }
            let key = event::read().unwrap();
            match key {  // todo add checks for state "in battle" before send char
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        continue
                    }
                    if key.code == KeyCode::Char(key_config.key_a) {
                        sender.send(APIQuery::BattleClick('a')).unwrap()
                    } else if key.code == KeyCode::Char(key_config.key_b) {
                        sender.send(APIQuery::BattleClick('b')).unwrap()
                    } else if key.code == KeyCode::Char(key_config.key_c) {
                        sender.send(APIQuery::BattleClick('c')).unwrap()
                    } else if key.code == KeyCode::Char(key_config.key_d) {
                        sender.send(APIQuery::BattleClick('d')).unwrap()
                    } else if key.code == KeyCode::Char(key_config.key_q) {
                        todo!()
                    } else if key.code == KeyCode::Down {
                        ui_sender.send(KeyToUI::Down).unwrap()
                    } else if key.code == KeyCode::Up {
                        ui_sender.send(KeyToUI::Up).unwrap()
                    } else if key.code == KeyCode::Enter {
                        ui_sender.send(KeyToUI::Choose).unwrap()
                    }
                }
                _ => {}
            }
        }
    }
}