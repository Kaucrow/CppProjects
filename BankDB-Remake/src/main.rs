#![allow(non_snake_case)]

mod model;
mod event;
mod update;
mod tui;
mod ui;

use anyhow::Result; 
use ratatui::{ backend::CrosstermBackend, Terminal };
use model::app::{App, Screen};
use update::update;
use event::EventHandler;
use tui::Tui;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "postgres://postgres:postgresPass@localhost:5432/bank";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let app = App::new();
    let mut app_arc = Arc::new(Mutex::new(app));

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(100, &app_arc);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    app_arc.lock().unwrap().enter_screen(Screen::Login);

    let mut _counter = 0;

    tui.draw(&mut app_arc)?;

    while !app_arc.lock().unwrap().should_quit {
        if let Ok(event) = tui.events.next() {
            //println!("{}", _counter);
            //_counter += 1;
            update(&mut app_arc, &pool, event).await.unwrap_or_else(|error| panic!("{}", error));
            tui.draw(&mut app_arc)?;
        }
    }

    tui.exit()?;

    Ok(())
}