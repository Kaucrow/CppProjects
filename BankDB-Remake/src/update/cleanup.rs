use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::{
    HELP_TEXT,
    model::{
        app::App,
        common::{Popup, Screen, InputMode, SideScreen},
    }
};

pub fn cleanup(app: &mut Arc<Mutex<App>>) -> Result<()> {
    let mut app_lock = app.lock().unwrap();

    if let Some(popup) = app_lock.active_popup {
        match popup {
            Popup::LoginSuccessful => {
                app_lock.input.0.reset();
                app_lock.input.1.reset();
            }
            Popup::Deposit | Popup::Withdraw => {
                app_lock.help_text = HELP_TEXT.client.main.to_string();
                app_lock.input.0.reset();
            }
            Popup::Transfer | Popup::ChangePsswd => {
                app_lock.help_text = HELP_TEXT.client.main.to_string();
                app_lock.input.0.reset();
                app_lock.input.1.reset();
            }
            Popup::FilterClients | Popup::AddClient => {
                app_lock.admin.cltfields_list_state.select(None);
                app_lock.admin.active_cltfield = None;
                app_lock.input.0.reset();
                app_lock.input.1.reset();
                if let Popup::AddClient = popup {
                    match app_lock.switch_popup {
                        Some(Popup::AddClientPsswd) => {
                            app_lock.active_popup = Some(Popup::AddClientPsswd);
                            app_lock.input_mode = InputMode::Editing(0);
                            return Ok(());
                        }
                        None =>
                            if app_lock.hold_popup { return Ok(()); }
                            else {
                                app_lock.admin.registered_cltfields.values_mut().for_each(|value| *value = None);
                            },
                        _ =>
                            panic!("popup {:?} can't switch to {:?}", popup, app_lock.switch_popup)
                    }
                }
                app_lock.help_text = HELP_TEXT.admin.main_left.to_string();
            }
            Popup::AddClientPsswd => {
                app_lock.input.0.reset();
                app_lock.admin.registered_cltfields.values_mut().for_each(|value| *value = None);
                app_lock.active_popup = Some(Popup::AddClientSuccess);
                return Ok(());
            }
            _ => {}
        }
    } else {
        match app_lock.active_screen {
            Screen::Client => {
                app_lock.client.actions_list_state.select(None);
            }
            Screen::Admin => {
                match app_lock.admin.active_sidescreen {
                    SideScreen::AdminClientTable => {
                        app_lock.admin.action_list_state.select(None);
                        app_lock.admin.clients_table_state.select(None);
                        app_lock.admin.viewing_clients = 0;
                        app_lock.admin.query_clients = String::from("SELECT * FROM clients");
                        for value in app_lock.admin.applied_filters.values_mut() {
                            *value = None;
                        }
                    }
                    SideScreen::AdminClientEdit => {
                        app_lock.admin.active_sidescreen = SideScreen::AdminClientTable
                    }
                }
            }
            _ => {}
        }
    }

    app_lock.hold_popup = false;
    app_lock.active_popup = None;

    Ok(())
}