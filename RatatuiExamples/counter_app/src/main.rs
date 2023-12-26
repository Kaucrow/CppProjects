/// Application
mod app;

/// Terminal events handler
mod event;

/// Widget renderer
mod ui;

/// Terminal user interface
mod tui;

/// Application updater
mod update;

use app::App;
use anyhow::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    // Create an application
    let mut app = App::new();

    // Initialize the terminal user interface
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Specify the tick rate for the draw
    let draw_tick_rate = Duration::from_millis(0);
    let mut last_draw_tick = Instant::now();
    let mut should_draw = true;

    // Start the main loop
    while !app.should_quit {
        if Instant::now() - last_draw_tick >= draw_tick_rate {
            should_draw = true;
            last_draw_tick = Instant::now();
        }

        if should_draw {
            // Render the user interface
            tui.draw(&mut app)?;
            should_draw = false;
        }

        // Handle events
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}