use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use crate::entities::models::app::AppState;
use crate::entities::models::menu::KeyToUI;
use crate::shared::api::threads::worker::APIQuery;

pub fn continuous_rendering(app_state: Arc<Mutex<AppState>>, sender: Sender<APIQuery>, receiver: Receiver<KeyToUI>) {
    loop {

    }
}