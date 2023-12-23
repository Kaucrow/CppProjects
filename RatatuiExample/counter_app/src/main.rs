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

fn main() -> Result<()> {
    // Create an application
    let mut app = App::new();

    // Initialize the terminal user interface
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop
    while !app.should_quit {
        // Render the user interface
        tui.draw(&mut app)?;
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