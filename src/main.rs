mod app;
mod shared;

mod entities;
mod pages;
mod features;

use std::sync::{Arc, mpsc, Mutex};
use std::{env, thread};
use color_eyre::Result;
use shared::api::threads::worker::work;
use crate::shared::config::app::read_config;

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
    thread::spawn(move || work(api_state, rx, app_config));

    Ok(())
}