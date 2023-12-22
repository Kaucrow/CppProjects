use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen
    }
};

use ratatui::{
    prelude::{CrosstermBackend, Terminal, Frame},
    widgets::Paragraph,
};

use anyhow::Result;

// If not using anyhow:
//type Err = Box<dyn std::error::Error>;
//type Result<T> = std::result::Result<T, Err>;

struct App {
    counter: i64,
    should_quit: bool
}

fn main() -> Result<()> {
    // Setup terminal
    startup()?;

    let result = run();

    // Teardown terminal before unwrapping Result of app run
    shutdown()?;

    result?;
    
    Ok(())
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) {
    f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
}

fn update(app: &mut App) -> Result<()> {
    // Check for keypresses every 250ms
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('k') => app.counter += 1,
                    Char('j') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    // Ratatui terminal
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Application state
    let mut app = App { counter: 0, should_quit: false };

    loop {
        // Application update
        update(&mut app)?;

        // Application render
        t.draw(|f| {
            ui(&app, f);
        })?;

        // Application exit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}