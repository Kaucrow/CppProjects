use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres};
use anyhow::Result;
use crate::{
    HELP_TEXT,
    event::Event,
    model::{
        common::{ScreenSection, Popup, InputMode, ListType},
        app::App,
    }
};

pub async fn update(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    match event {
        Event::NextListItem(list_type) => {
            app.lock().unwrap().next_list_item(list_type);
            Ok(())
        },
        Event::PreviousListItem(list_type) => {
            app.lock().unwrap().previous_list_item(list_type);
            Ok(())
        },
        Event::NextTableItem(table_type) => {
            app.lock().unwrap().next_table_item(table_type, pool).await?;
            Ok(())
        },
        Event::PreviousTableItem(table_type) => {
            app.lock().unwrap().previous_table_item(table_type, pool).await?;
            Ok(())
        },
        Event::SelectAction(list_type) => {
            let mut app_lock = app.lock().unwrap();
            let (list_state, popups) = match list_type {
                ListType::ClientAction => (&app_lock.client.action_list_state, &app_lock.client.popups),
                ListType::AdminAction => (&app_lock.admin.action_list_state, &app_lock.admin.popups),
                _ => panic!()
            };

            if let Some(selected) = list_state.selected() {
                app_lock.active_popup = Some(*popups.get(&selected).unwrap_or_else(|| panic!("popup not found in popups HashMap")));
                match app_lock.active_popup.unwrap() {
                    Popup::Deposit | Popup::Withdraw | Popup::Transfer | Popup::ChangePsswd
                    => {
                        app_lock.input_mode = InputMode::Editing(0);
                        app_lock.help_text = match app_lock.active_popup.unwrap() {
                            Popup::Deposit => HELP_TEXT.client.deposit.to_string(),
                            Popup::Withdraw => HELP_TEXT.client.withdraw.to_string(),
                            Popup::Transfer => HELP_TEXT.client.transfer.to_string(),
                            Popup::ChangePsswd => HELP_TEXT.client.change_psswd.to_string(),
                            _ => panic!()
                        }
                    }
                    Popup::FilterClients => {
                        app_lock.admin.popup_screen_section = ScreenSection::Left;
                        app_lock.help_text = HELP_TEXT.admin.filter_left.to_string();
                    }
                    Popup::AddClient => {
                        app_lock.admin.popup_screen_section = ScreenSection::Left;
                        app_lock.help_text = HELP_TEXT.admin.add_client_left.to_string();
                    }
                    _ => {}
                }
            }
            Ok(())
        },
        _ => panic!("An event of type {:?} was passed to the wrong update function", event)
    }
}