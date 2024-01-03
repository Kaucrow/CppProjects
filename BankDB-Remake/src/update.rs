use crossterm::event::{Event as CrosstermEvent, KeyEventKind, KeyCode};
use std::sync::{Arc, Mutex};
use tui_input::{backend::crossterm::EventHandler, Input};
use sqlx::{Row, Pool, Postgres, FromRow};
use rust_decimal::Decimal;
use bcrypt::verify;
use anyhow::Result;
use crate::{
    event::{ 
        Event,
        InputBlacklist,
    },
    model::{
        app::{
            App,
            Screen,
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
        Event::ExitPopup => {
            app.lock().unwrap().active_popup = None;
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
            app.lock().unwrap().enter_screen(Screen::Client);
            Ok(())
        }
        Event::SwitchInput => {
            let mut app_lock = app.lock().unwrap();

            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 { app_lock.input_mode = InputMode::Editing(1) }
                else { app_lock.input_mode = InputMode::Editing(0) }
            }
            Ok(())
        }
        Event::KeyInput(key_event, blacklist) => {
            let mut app_lock = app.lock().unwrap();

            let field = match app_lock.input_mode {
                InputMode::Editing(field) => field,
                InputMode::Normal => panic!("KeyInput event fired when InputMode was normal")
            };

            if let KeyCode::Char(char) = key_event.code {
                match blacklist {
                    InputBlacklist::None => {}
                    InputBlacklist::Money => {
                        if char != '.' {
                            if !char.is_numeric() {
                                return Ok(());
                            }
                        } else {
                            if field == 0 {
                                if app_lock.input.0.value().contains('.') {
                                    return Ok(())
                                } 
                            } else {
                                if app_lock.input.1.value().contains('.') {
                                    return Ok(())
                                }
                            }
                        }
                    }
                    _ => { unimplemented!("blacklist isn't implemented") }
                }
            };
 
            if field == 0 { app_lock.input.0.handle_event(&CrosstermEvent::Key(key_event)); }
            else { app_lock.input.1.handle_event(&CrosstermEvent::Key(key_event)); }
            Ok(())
        },
        Event::NextClientAction => {
            app.lock().unwrap().next_client_action();
            Ok(())
        },
        Event::PreviousClientAction => {
            app.lock().unwrap().previous_client_action();
            Ok(())
        },
        Event::SelectAction => {
            let mut app_lock = app.lock().unwrap();
            if let Some(selected) = app_lock.client_action_list_state.selected() {
                app_lock.active_popup = Some(*app_lock.client_popups.get(&selected).unwrap_or_else(|| panic!("popup not found in client_popups")));
                match app_lock.active_popup.unwrap() {
                    Popup::Deposit => app_lock.input_mode = InputMode::Editing(0),
                    _ => {}
                }
            }
            Ok(())
        }
        Event::Deposit => {
            let mut app_lock = app.lock().unwrap();
            let deposit_input = Decimal::from_str_exact(app_lock.input.0.value())?;
            app_lock.active_user.as_mut().unwrap().balance += deposit_input;
            sqlx::query("UPDATE clients SET balance = $1 WHERE username = $2")
                .bind(&app_lock.active_user.as_ref().unwrap().balance)
                .bind(&app_lock.active_user.as_ref().unwrap().username)
                .execute(pool)
                .await?;
            app_lock.active_popup = None;
            Ok(())
        }
        _ => { Ok(()) }
    }
}