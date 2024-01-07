use std::sync::{Arc, Mutex};
use crossterm::event::{Event as CrosstermEvent, KeyCode};
use tui_input::backend::crossterm::EventHandler;
use sqlx::{Row, Pool, Postgres, FromRow};
use anyhow::Result;
use bcrypt::verify;
use crate::{
    event::{
        Event,
        InputBlacklist,
    },
    model::{
        app::{
        App,
        InputMode,
        Screen,
        ScreenSection,
        Popup,
        TimeoutType,
        ScreenSectionType,
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
        Event::ExitPopup => {
            let mut app_lock = app.lock().unwrap();
            app_lock.active_popup = None;
            app_lock.input.0.reset();
            app_lock.input.1.reset();
            app_lock.hold_popup = false;
            for value in app_lock.admin.applied_filters.values_mut() {
                *value = None;
            }
            app_lock.admin.filter_list_state.select(None);
            app_lock.admin.active_filter = None;
            match app_lock.curr_screen {
                Screen::Client => app_lock.help_text = "Choose an action to perform. Press Esc to go back.",
                _ => {}
            }
            Ok(())
        },
        Event::EnterScreen(screen) => {
            app.lock().unwrap().enter_screen(screen);
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
        Event::SwitchInput => {
            let mut app_lock = app.lock().unwrap();

            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 { app_lock.input_mode = InputMode::Editing(1) }
                else { app_lock.input_mode = InputMode::Editing(0) }
            }
            Ok(())
        },
        Event::SwitchScreenSection(screen_section_type) => {
            let mut app_lock = app.lock().unwrap();

            let curr_screen_section = match screen_section_type {
                ScreenSectionType::AdminMain => &mut app_lock.curr_screen_section,
                ScreenSectionType::AdminFilters => &mut app_lock.admin.filter_screen_section,
            };

            if let ScreenSection::Left = curr_screen_section {
                *curr_screen_section = ScreenSection::Right;
            } else {
                *curr_screen_section = ScreenSection::Left;
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
                    InputBlacklist::Alphabetic => {
                        if !char.is_alphabetic() && char != ' ' {
                            return Ok(())
                        }
                    }
                    InputBlacklist::Numeric => {
                        if !char.is_numeric() {
                            return Ok(())
                        }
                    }
                }
            };
 
            if field == 0 { app_lock.input.0.handle_event(&CrosstermEvent::Key(key_event)); }
            else { app_lock.input.1.handle_event(&CrosstermEvent::Key(key_event)); }
            Ok(())
        },
        _ => panic!("An event of type {:?} was passed to the common update function", event)
    }
}