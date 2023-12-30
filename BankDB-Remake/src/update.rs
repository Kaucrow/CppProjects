//use crossterm::event::{Event as CrosstermEvent, KeyEventKind, KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use std::sync::{Arc, Mutex};
use tui_input::backend::crossterm::EventHandler;
use sqlx::{Row, Pool, Postgres, Executor};
use anyhow::Result;

use crate::model::{App, InputMode};
use crate::event::Event;

pub async fn update(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    match event {
        Event::Quit => {
            app.lock().unwrap().should_quit = true;
            Ok(())
        },
        Event::TryLogin => {
            let name: String = app.lock().unwrap().input.0.value().to_string();/*.split_whitespace().map(|word| {
                    let lowercase_word = word.to_lowercase();
                    let mut chars = lowercase_word.chars();
                    match chars.next() {
                        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
                        None => String::new(),
                    }
                }).collect::<Vec<String>>()
                .join(" ");*/

            let password: String = app.lock().unwrap().input.1.value().to_string();

            if let Some(res) = sqlx::query("SELECT name, password FROM clients WHERE LOWER(name) = LOWER($1)")
                .bind(&name)
                .fetch_optional(pool)
                .await? {
                    let res_name: String = res.try_get("name")?;
                    let res_password: String = res.try_get("password")?;
                    if password == res_password {
                        println!("{:?}, {:?}", res_name, res_password);
                        return Ok(());
                    }
                }
            
            println!("WRONG NAME OR PASSWORD");
            Ok(())
            //let app_lock = app.lock().unwrap();
            //todo!("Login input: [Name: {}], [Password: {}]", app_lock.input.0.value(), app_lock.input.1.value());
        },
        Event::SwitchInput => {
            let mut app_lock = app.lock().unwrap();
            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 { app_lock.input_mode = InputMode::Editing(1) }
                else { app_lock.input_mode = InputMode::Editing(0) }
            }
            Ok(())
        }
        Event::Key(key_event) => {
            let mut app_lock = app.lock().unwrap();
            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 { app_lock.input.0.handle_event(&CrosstermEvent::Key(key_event)); }
                else { app_lock.input.1.handle_event(&CrosstermEvent::Key(key_event)); }
            }
            Ok(())
        },
        _ => { Ok(()) }
    }
}