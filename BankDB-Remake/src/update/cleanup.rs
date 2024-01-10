use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::{
    HELP_TEXT,
    model::{
        app::App,
        common::{Popup, Screen, ScreenSection},
    }
};

pub fn cleanup(app: &mut Arc<Mutex<App>>) -> Result<()> {
    let mut app_lock = app.lock().unwrap();

    if let Some(popup) = app_lock.active_popup {
        app_lock.hold_popup = false;
        match popup {
            Popup::LoginSuccessful => {
                app_lock.input.0.reset();
                app_lock.input.1.reset();
            }
            Popup::Deposit | Popup::Withdraw => {
                app_lock.help_text = HELP_TEXT.client.main;
                app_lock.input.0.reset();
            }
            Popup::Transfer | Popup::ChangePsswd => {
                app_lock.help_text = HELP_TEXT.client.main;
                app_lock.input.0.reset();
                app_lock.input.1.reset();
            }
            Popup::FilterClients => {
                app_lock.help_text = HELP_TEXT.admin.main_left;
                app_lock.admin.filter_list_state.select(None);
                app_lock.admin.active_filter = None;
                app_lock.input.0.reset();
                app_lock.input.1.reset();
            }
            Popup::AddClient => {
                app_lock.help_text = HELP_TEXT.admin.main_left;
                app_lock.input.0.reset();
                app_lock.input.1.reset();
            }
            _ => {}
        }
    } else {
        match app_lock.active_screen {
            Screen::Client => {
                app_lock.client.action_list_state.select(None);
            }
            Screen::Admin => {
                app_lock.admin.action_list_state.select(None);
                app_lock.admin.client_table_state.select(None);
                app_lock.admin.viewing_clients = 0;
                app_lock.admin.query_clients = String::from("SELECT * FROM clients");
                for value in app_lock.admin.applied_filters.values_mut() {
                    *value = None;
                }
            }
            _ => {}
        }
    }

    app_lock.active_popup = None;

    Ok(())
}