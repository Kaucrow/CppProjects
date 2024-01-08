mod login;
mod admin;
mod client;
mod common_fn;

use std::sync::{Arc, Mutex};
use ratatui::prelude::Frame;
use crate::model::{common::Screen, app::App};

pub fn render(app: &mut Arc<Mutex<App>>, f: &mut Frame) {
    let curr_screen = app.lock().unwrap().curr_screen.clone();

    match curr_screen {
        Screen::Login => login::render(app, f),
        Screen::Client => client::render(app, f),
        Screen::Admin => admin::render(app, f)
    }
}