//use crossterm::event::{Event as CrosstermEvent, KeyEventKind, KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use std::sync::{Arc, Mutex};
use tui_input::backend::crossterm::EventHandler;

use crate::model::{App, InputMode};
use crate::event::Event;

pub fn update(app: &mut Arc<Mutex<App>>, event: Event) {
    match event {
        Event::Quit => {
            app.lock().unwrap().should_quit = true;
        },
        Event::TryLogin => {
            let app_lock = app.lock().unwrap();
            todo!("Login input: [Name: {}], [Password: {}]", app_lock.input.0.value(), app_lock.input.1.value());
        },
        Event::SwitchInput => {
            let mut app_lock = app.lock().unwrap();
            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 { app_lock.input_mode = InputMode::Editing(1) }
                else { app_lock.input_mode = InputMode::Editing(0) }
            }
        }
        Event::Key(key_event) => {
            let mut app_lock = app.lock().unwrap();
            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 { app_lock.input.0.handle_event(&CrosstermEvent::Key(key_event)); }
                else { app_lock.input.1.handle_event(&CrosstermEvent::Key(key_event)); }
            }
        },
        _ => {}
    }
}