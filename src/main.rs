mod app;
mod shared;
mod application;
mod entities;
mod pages;
mod features;
mod widgets;
mod ui;
mod main_old;

use std::sync::{Arc, mpsc, Mutex};
use std::{env, thread};
use std::time::Duration;
use color_eyre::Result;
use shared::api::threads::worker::work;
use crate::entities::models::app::AppState;
use crate::features::clicks::take_keys;
use crate::shared::config::app::read_config;
use crate::shared::config::clicks::KeyConfig;
use crate::widgets::display::continuous_rendering;

fn main() -> Result<()> {
    color_eyre::install()?;
    // 1. Starting display thread
    // 2. Starting keypress handler thread
    // 3. Starting client thread
    let app_config = read_config(env::var("RPGRAM_CONFIG").unwrap());
    let app_state = AppState::new();
    let shared_app = Arc::new(Mutex::new(app_state));
    let api_state = Arc::clone(&shared_app);
    let (api_tx, api_rx) = mpsc::channel();
    let api_tx_strokes = api_tx.clone();
    let (ui_tx, ui_rx) = mpsc::channel();
    let (exit_tx, exit_rx) = mpsc::channel();
    let api_handle = thread::spawn(move || work(api_state, api_rx, app_config));
    let stroke_listener = thread::spawn(move || take_keys(api_tx_strokes, ui_tx, KeyConfig::default(), exit_tx));
    let ui_handle = thread::spawn(move || continuous_rendering(shared_app, api_tx, ui_rx));
    exit_rx.recv().unwrap();
    api_handle.join().unwrap();
    stroke_listener.join().unwrap();
    ui_handle.join().unwrap();
    ratatui::restore();
    Ok(())
}