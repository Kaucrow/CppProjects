//use crossterm::event::{Event as CrosstermEvent, KeyEventKind, KeyCode, KeyEvent, KeyModifiers};
use std::sync::{Arc, Mutex};
use crate::model::App;
use crate::event::Event;

pub fn update(app: &mut Arc<Mutex<App>>, event: Event) {
    match event {
        Event::Quit => {
            app.lock().unwrap().should_quit = true;
        },
        _ => {}
    }
}