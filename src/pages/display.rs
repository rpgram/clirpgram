use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::entities::models::app::AppState;
use crate::shared::api::threads::worker::APIQuery;

// pub fn work(app_state: Arc<Mutex<AppState>>) {
//     todo!()
// }