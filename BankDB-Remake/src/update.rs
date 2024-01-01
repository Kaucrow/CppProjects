use crossterm::event::Event as CrosstermEvent;
use std::sync::{Arc, Mutex};
use tui_input::backend::crossterm::EventHandler;
use sqlx::{Row, Pool, Postgres, FromRow};
use bcrypt::verify;
use anyhow::Result;
use crate::{
    event::Event,
    model::{
        app::{
            App,
            Popup,
            InputMode,
            TimeoutType,
        },
        client::Client,
    }
};

pub async fn update(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    match event {
        Event::Quit => {
            app.lock().unwrap().should_quit = true;
            Ok(())
        },
        Event::TimeoutStep(timeout_type) => {
            app.lock().unwrap().update_timeout_counter(timeout_type);
            Ok(())
        },
        Event::TryLogin => {
            if app.lock().unwrap().failed_logins == 3 {
                return Ok(());
            }

            let username: String = app.lock().unwrap().input.0.value().to_string();
            let password: String = app.lock().unwrap().input.1.value().to_string();

            if let Some(res) = sqlx::query("SELECT * FROM clients WHERE username = $1")
                .bind(&username)
                .fetch_optional(pool)
                .await? {
                    let password_hash: String = res.try_get("password")?;

                    if verify(&password, &password_hash).unwrap_or_else(|error| panic!("{}", error)) {
                        let mut app_lock = app.lock().unwrap();
                        app_lock.active_user = Some(Client::from_row(&res)?);
                        app_lock.active_user.as_mut().unwrap().update_transaction(pool).await?;
                        app_lock.active_popup = Some(Popup::LoginSuccessful);

                        //todo!("login successful, but not yet implemented. USER: {:?}", app_lock.active_user);
                        return Ok(());
                    }
                }

            let mut app_lock = app.lock().unwrap();
            app_lock.failed_logins += 1;
            
            if app_lock.failed_logins == 3 {
                app_lock.add_timeout(30, 1000, TimeoutType::Login);
            }
            Ok(())
        },
        Event::EnterClientScreen => {
            panic!("entering client screen");
        }
        Event::SwitchInput => {
            let mut app_lock = app.lock().unwrap();

            if app_lock.active_popup.is_some() { return Ok(()); }

            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 { app_lock.input_mode = InputMode::Editing(1) }
                else { app_lock.input_mode = InputMode::Editing(0) }
            }
            Ok(())
        }
        Event::Key(key_event) => {
            let mut app_lock = app.lock().unwrap();
            
            if app_lock.active_popup.is_some() { return Ok(()); }

            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 { app_lock.input.0.handle_event(&CrosstermEvent::Key(key_event)); }
                else { app_lock.input.1.handle_event(&CrosstermEvent::Key(key_event)); }
            }
            Ok(())
        },
        _ => { Ok(()) }
    }
}