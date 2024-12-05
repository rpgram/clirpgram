mod app;
mod shared;

mod entities;
mod pages;
mod features;
mod widgets;
mod ui;
mod main_old;

use std::sync::{Arc, mpsc, Mutex};
use std::{env, thread};
use color_eyre::Result;
use shared::api::threads::worker::work;
use crate::features::clicks::take_keys;
use crate::shared::config::app::read_config;
use crate::shared::config::clicks::KeyConfig;

fn main() -> Result<()> {
    color_eyre::install()?;
    // 1. Starting display thread
    // 2. Starting keypress handler thread
    // 3. Starting client thread
    let app_config = read_config(env::var("RPGRAM_CONFIG").unwrap());
    let app_state = todo!();
    let mut shared_app = Arc::new(Mutex::new(app_state));
    let mut api_state = Arc::clone(&shared_app);
    let (tx, rx) = mpsc::channel();
    let (ui_tx, ui_rx) = mpsc::channel();
    thread::spawn(move || work(api_state, rx, app_config));
    thread::spawn(move || take_keys(tx.clone(), ui_tx, KeyConfig::default()));
    ratatui::restore();
    Ok(())
}