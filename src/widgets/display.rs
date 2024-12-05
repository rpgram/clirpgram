use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use crate::entities::models::app::AppState;
use crate::entities::models::menu::KeyToUI;
use crate::shared::api::threads::worker::APIQuery;

pub fn continuous_rendering(_app_state: Arc<Mutex<AppState>>, _sender: Sender<APIQuery>, receiver: Receiver<KeyToUI>) {
    loop {
        let rec_res = receiver.try_recv();
        if let Ok(key) = rec_res {
            match key {
                KeyToUI::Up => {}
                KeyToUI::Down => {}
                KeyToUI::Choose => {}
                KeyToUI::Shutdown => {return;}
            }
        }
    }
}