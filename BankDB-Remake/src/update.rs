use crossterm::event::{Event as CrosstermEvent, KeyCode};
use std::sync::{Arc, Mutex};
use tui_input::backend::crossterm::EventHandler;
use sqlx::{Row, Pool, Postgres, FromRow};
use rust_decimal::Decimal;
use bcrypt::{verify, hash};
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
            ScreenSection,
            Popup,
            InputMode,
            ListType,
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
                Screen::Client => app_lock.help_text = "Choose an action to perform. Press Esc to go back.",
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
                        app_lock.client.active = Some(Client::from_row(&res)?);
                        app_lock.client.active.as_mut().unwrap().update_transaction(pool).await?;
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
        Event::EnterScreen(screen) => {
            app.lock().unwrap().enter_screen(screen);
            Ok(())
        },
        Event::SwitchInput => {
            let mut app_lock = app.lock().unwrap();

            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 { app_lock.input_mode = InputMode::Editing(1) }
                else { app_lock.input_mode = InputMode::Editing(0) }
            }
            Ok(())
        },
        Event::SwitchScreenSection => {
            let mut app_lock = app.lock().unwrap();

            if let ScreenSection::Left = app_lock.curr_screen_section {
                app_lock.curr_screen_section = ScreenSection::Right;
            } else {
                app_lock.curr_screen_section = ScreenSection::Left;
            }
            Ok(())
        },
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
        Event::NextListItem(list_type) => {
            app.lock().unwrap().next_list_item(list_type);
            Ok(())
        },
        Event::PreviousListItem(list_type) => {
            app.lock().unwrap().previous_list_item(list_type);
            Ok(())
        },
        Event::SelectAction(list_type) => {
            let mut app_lock = app.lock().unwrap();
            let (list_state, popups) = match list_type {
                ListType::ClientAction => (&app_lock.client.action_list_state, &app_lock.client.popups),
                ListType::AdminAction => (&app_lock.admin.action_list_state, &app_lock.admin.popups)
            };

            if let Some(selected) = list_state.selected() {
                app_lock.active_popup = Some(*popups.get(&selected).unwrap_or_else(|| panic!("popup not found in popups HashMap")));
                match app_lock.active_popup.unwrap() {
                    Popup::Deposit | Popup::Withdraw | Popup::Transfer | Popup::ChangePsswd => app_lock.input_mode = InputMode::Editing(0),
                    _ => {}
                }
            }
            Ok(())
        },
        Event::Deposit | Event::Withdraw => {
            modify_balance(app, pool, event).await?;
            let mut app_lock = app.lock().unwrap();
            app_lock.input.0.reset();
            app_lock.active_popup = None;
            Ok(())
        },
        Event::Transfer => {
            let beneficiary = app.lock().unwrap().input.1.value().to_string();
            {
                let mut app_lock = app.lock().unwrap();
                if beneficiary == app_lock.client.active.as_ref().unwrap().username {
                    app_lock.help_text = "You can't transfer money to yourself.";
                    app_lock.hold_popup = true;
                    return Ok(());
                }
            }

            match sqlx::query("SELECT * FROM clients WHERE username = $1")
                .bind(&beneficiary)
                .fetch_optional(pool)
                .await? {
                    Some(_) => {
                        let prev_balance = app.lock().unwrap().client.active.as_ref().unwrap().balance.clone();

                        modify_balance(app, pool, event).await?;

                        let mut app_lock = app.lock().unwrap();
                        if app_lock.client.active.as_ref().unwrap().balance != prev_balance {
                            sqlx::query("UPDATE clients SET balance = balance + $1 WHERE username = $2")
                            .bind(Decimal::from_str_exact(app_lock.input.0.value()).unwrap())
                            .bind(beneficiary)
                            .execute(pool)
                            .await?;

                            app_lock.input.0.reset();
                            app_lock.input.1.reset();
                            app_lock.active_popup = None;
                        }
                    },
                    None => {
                        let mut app_lock = app.lock().unwrap();
                        app_lock.help_text = "The beneficiary doesn't exist.";
                        app_lock.hold_popup = true;
                    }
                }
            Ok(())
        },
        Event::ChangePasswd => {
            let mut app_lock = app.lock().unwrap();
            let curr_passwd = &app_lock.client.active.as_ref().unwrap().password_hash;
            let curr_passwd_input = app_lock.input.0.value();

            if verify(curr_passwd_input, curr_passwd).unwrap_or_else(|error| panic!("{}", error)) {
                let new_password_hash = hash(
                    app_lock.input.1.value(), 4).unwrap_or_else(|error| panic!("{}", error
                ));

                sqlx::query("UPDATE clients SET password = $1 WHERE username = $2")
                    .bind(&new_password_hash)
                    .bind(&app_lock.client.active.as_ref().unwrap().username)
                    .execute(pool)
                    .await?;

                app_lock.client.active.as_mut().unwrap().password_hash = new_password_hash;

                app_lock.input.0.reset();
                app_lock.input.1.reset();
                app_lock.active_popup = None;
            } else {
                app_lock.help_text = "Incorrect current password.";
                app_lock.hold_popup = true;
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
        app_lock.client.active.as_mut().unwrap().balance += amount_input;
    } else {
        if amount_input > app_lock.client.active.as_ref().unwrap().balance {
            app_lock.help_text = "You don't have enough money.";
            app_lock.hold_popup = true;
            return Ok(())
        } else {
            app_lock.client.active.as_mut().unwrap().balance -= amount_input;
        }
    }

    sqlx::query("UPDATE clients SET balance = $1 WHERE username = $2")
        .bind(&app_lock.client.active.as_ref().unwrap().balance)
        .bind(&app_lock.client.active.as_ref().unwrap().username)
        .execute(pool)
        .await?;

    Ok(())
}