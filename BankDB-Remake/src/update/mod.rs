mod common;
mod list;
mod client;
mod admin;
mod common_fn;

use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres};
use anyhow::Result;
use crate::{
    event::Event,
    model::app::App,
};

pub async fn update(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    match event {
        Event::Quit | Event::TimeoutStep(_) | Event::ExitPopup | Event::EnterScreen(_) |
        Event::TryLogin | Event::SwitchInput | Event::SwitchScreenSection(_) | Event::KeyInput(..)
        => common::update(app, pool, event).await,

        Event::NextListItem(_) | Event::PreviousListItem(_) | Event::SelectAction(_)
        => list::update(app, pool, event).await,

        Event::Deposit | Event::Withdraw | Event::Transfer | Event::ChangePasswd
        => client::update(app, pool, event).await,

        Event::EditFilter | Event::RegisterFilter | Event::SwitchButton
        => admin::update(app, pool, event).await,

        _ => { Ok(()) }
    }
}