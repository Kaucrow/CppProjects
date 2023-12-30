//use crossterm::event::{Event as CrosstermEvent, KeyEventKind, KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use std::sync::{Arc, Mutex};
use tui_input::backend::crossterm::EventHandler;

use crate::model::App;
use crate::event::Event;

pub fn update(app: &mut Arc<Mutex<App>>, event: Event) {
    match event {
        Event::Quit => {
            app.lock().unwrap().should_quit = true;
        },
        Event::TryLogin => {
            todo!("Login input: {}", app.lock().unwrap().input.value.value());
        },
        Event::Key(key_event) => {
            app.lock().unwrap().input.value.handle_event(&CrosstermEvent::Key(key_event));
        },
        _ => {}
    }
}