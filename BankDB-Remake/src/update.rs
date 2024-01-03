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
            let mut app_lock = app.lock().unwrap();
            app_lock.active_popup = None;
            app_lock.input.0.reset();
            app_lock.input.1.reset();
            app_lock.hold_popup = false;
            match app_lock.curr_screen {
                Screen::Client => app_lock.help_text = String::from("Choose an action to perform."),
                _ => {}
            }
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
                        let input_value = {
                            if field == 0 {
                                app_lock.input.0.value()
                            } else {
                                app_lock.input.1.value()
                            }
                        };

                        if char != '.' {
                            if !char.is_numeric() {
                                return Ok(());
                            } else {
                                if let Some(dot_index) = input_value.find('.') {
                                    if input_value[dot_index + 1..].len() == 2 { return Ok(()) }
                                }
                            }
                        } else {
                            if app_lock.input.0.value().contains('.') {
                                return Ok(())
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
                    Popup::Deposit | Popup::Withdraw | Popup::Transfer => app_lock.input_mode = InputMode::Editing(0),
                    _ => {}
                }
            }
            Ok(())
        },
        Event::Deposit | Event::Withdraw => {
            modify_balance(app, pool, event).await?;
            app.lock().unwrap().input.0.reset();
            Ok(())
        },
        Event::Transfer => {
            let beneficiary = app.lock().unwrap().input.1.value().to_string();
            {
                let mut app_lock = app.lock().unwrap();
                if beneficiary == app_lock.active_user.as_ref().unwrap().username {
                    app_lock.help_text = String::from("You can't transfer money to yourself.");
                    app_lock.hold_popup = true;
                    return Ok(());
                }
            }

            match sqlx::query("SELECT * FROM clients WHERE username = $1")
                .bind(&beneficiary)
                .fetch_optional(pool)
                .await? {
                    Some(_) => {
                        let prev_balance = app.lock().unwrap().active_user.as_ref().unwrap().balance.clone();

                        modify_balance(app, pool, event).await?;

                        let mut app_lock = app.lock().unwrap();
                        if app_lock.active_user.as_ref().unwrap().balance != prev_balance {
                            sqlx::query("UPDATE clients SET balance = balance + $1 WHERE username = $2")
                            .bind(Decimal::from_str_exact(app_lock.input.0.value()).unwrap())
                            .bind(beneficiary)
                            .execute(pool)
                            .await?;
                        }

                        app_lock.input.0.reset();
                        app_lock.input.1.reset();
                    }
                    None => {
                        let mut app_lock = app.lock().unwrap();
                        app_lock.help_text = String::from("The beneficiary doesn't exist.");
                        app_lock.hold_popup = true;
                    }
                }
            Ok(())
        },
        _ => { Ok(()) }
    }
}

async fn modify_balance(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    let mut app_lock = app.lock().unwrap();
    let amount_input = Decimal::from_str_exact(app_lock.input.0.value())?;

    if let Event::Deposit = event {
        app_lock.active_user.as_mut().unwrap().balance += amount_input;
    } else {
        if amount_input > app_lock.active_user.as_ref().unwrap().balance {
            app_lock.help_text = String::from("You don't have enough money.");
            app_lock.hold_popup = true;
            return Ok(())
        } else {
            app_lock.active_user.as_mut().unwrap().balance -= amount_input;
        }
    }

    sqlx::query("UPDATE clients SET balance = $1 WHERE username = $2")
        .bind(&app_lock.active_user.as_ref().unwrap().balance)
        .bind(&app_lock.active_user.as_ref().unwrap().username)
        .execute(pool)
        .await?;

    app_lock.active_popup = None;
    Ok(())
}