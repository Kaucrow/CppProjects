mod common;
mod list;
mod client;
mod admin;
mod common_fn;
mod cleanup;

use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres};
use anyhow::Result;
use crate::{
    event::Event,
    model::app::App,
};

pub async fn update(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    match event {
        Event::Quit | Event::TimeoutStep(_) | Event::EnterScreen(_) | Event::TryLogin |
        Event::SwitchInput | Event::SwitchScreenSection(_) | Event::KeyInput(..)
        => common::update(app, pool, event).await,

        Event::NextListItem(_) | Event::PreviousListItem(_) | Event::SelectAction(_) |
        Event::NextTableItem(_) | Event::PreviousTableItem(_)
        => list::update(app, pool, event).await,

        Event::Deposit | Event::Withdraw | Event::Transfer | Event::ChangePasswd
        => client::update(app, pool, event).await,

        Event::EditCltData | Event::RegisterCltData(_) | Event::ApplyFilters |
        Event::CheckAddClient | Event::AddClient | Event::SwitchButton
        => admin::update(app, pool, event).await,

        Event::Cleanup
        => cleanup::cleanup(app),

        Event::Resize
        => Ok(()),

        _ => panic!("received event {:?} without assigned function", event)
    }
}